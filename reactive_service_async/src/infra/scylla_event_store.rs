use reactive_service_domain::aggregate_root::SequencedEvent;
use scylla::{IntoTypedRows, Session, SessionBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use crate::order_service::EventsJournal;

pub struct ScyllaEventStore { session: Session }

impl ScyllaEventStore {
    pub async fn new(contact_point: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let session = SessionBuilder::new()
            .known_node(contact_point)
            .build()
            .await?;

        // Create the keyspace "ddd" if it doesn't exist
        session.query(r#"
                CREATE KEYSPACE IF NOT EXISTS ddd
                WITH REPLICATION = {
                    'class': 'SimpleStrategy',
                    'replication_factor': 1
                };"#, (), ).await?;

        session.use_keyspace("ddd", false).await?;

        // Create the "events" table if it doesn't exist
        session.query(r#"
                CREATE TABLE IF NOT EXISTS events (
                    entity_id BIGINT,
                    sequence_number BIGINT,
                    event_payload TEXT,
                    PRIMARY KEY (entity_id, sequence_number)
                );"#, (), ).await?;

        Ok(Self { session })
    }
}

impl<E: Serialize + DeserializeOwned> EventsJournal<E> for ScyllaEventStore {
    async fn persist_event(&self, entity_id: i64, evt_w_seq: &SequencedEvent<E>) -> Result<(), &'static str> {
        let query = "INSERT INTO events (entity_id, sequence_number, event_payload) VALUES (?, ?, ?)";
        let values = (entity_id, evt_w_seq.sequence_number, serde_json::to_string(&evt_w_seq.event).unwrap());

        self.session.query(query, &values).await.map_err(|_| "Failed to persist event")?;
        Ok(())
    }

    async fn retrieve_events(&self, entity_id: i64) -> Result<Vec<SequencedEvent<E>>, &'static str> {
        let query = "SELECT sequence_number, event_payload FROM events WHERE entity_id = ? ORDER BY sequence_number ASC";
        let mut events = Vec::new();

        if let Some(rows) = self.session.query(query, (entity_id,)).await.unwrap().rows {
            for row in rows.into_typed::<(i64, String)>() {
                let (sequence_number, event_payload) = row.map_err(|_| "Deserialization Error")?;
                let event: E = serde_json::from_str(&event_payload).map_err(|_| "Deserialization Error")?;
                events.push(SequencedEvent { sequence_number, event });
            }
        }

        Ok(events)
    }
}


