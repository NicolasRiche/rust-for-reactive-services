use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;
use reactive_service_domain::aggregate_root::SequencedEvent;
use tokio_postgres::{NoTls, Client};

use crate::order_service::EventsJournal;

pub struct PostgresEventStore { client: Client }

impl PostgresEventStore {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Connect to the database.
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client.execute(
            "CREATE TABLE IF NOT EXISTS events (
                entity_id BIGINT NOT NULL,
                sequence_number BIGINT NOT NULL,
                payload TEXT NOT NULL,
                PRIMARY KEY(entity_id, sequence_number)
            )",
            &[],
        ).await?;

        Ok(Self { client })
    }
}

#[async_trait]
impl<E: Serialize + DeserializeOwned + Send + Sync> EventsJournal<E> for PostgresEventStore {

    async fn persist_event(&mut self, entity_id: i64, seq_event: &SequencedEvent<E>) -> Result<(), &'static str> {
        let serialized_event = serde_json::to_string(&seq_event.event).map_err(|_| "Failed to serialize event")?;
        self.client.execute(
            "INSERT INTO events (entity_id, sequence_number, payload) VALUES ($1, $2, $3)",
            &[&entity_id, &seq_event.sequence_number, &serialized_event],
        ).await.map_err(|_| "Failed to persist event")?;
        Ok(())
    }

    async fn retrieve_events(&mut self, entity_id: i64) -> Result<Vec<SequencedEvent<E>>, &'static str> {
        let rows = self.client
            .query("SELECT sequence_number, payload FROM events WHERE entity_id = $1 ORDER BY sequence_number ASC", &[&entity_id])
            .await
            .map_err(|_| "Failed to retrieve events")?;

        rows.iter()
            .map(|row| {
                let sequence_number: i64 = row.get(0);
                let event_payload: String = row.get(1);
                let event: E = serde_json::from_str(&event_payload).map_err(|_| "Failed to deserialize event")?;

                Ok(SequencedEvent {
                    sequence_number,
                    event,
                })
            })
            .collect()
    }
}
