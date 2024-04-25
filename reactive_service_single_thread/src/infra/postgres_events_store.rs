use std::marker::PhantomData;
use serde::Serialize;
use serde::de::DeserializeOwned;
use postgres::{Client, NoTls};
use reactive_service_domain::aggregate_root::SequencedEvent;
use crate::order_service::EventsJournal;

pub struct PostgresEventStore<E> {
    client: Client,
    _marker: PhantomData<E>,
}

impl <E> PostgresEventStore<E> {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {

        let mut client = Client::connect("host=localhost user=postgres password=postgres", NoTls)?;

        client.execute(
            "CREATE TABLE IF NOT EXISTS events (
                entity_id BIGINT NOT NULL,
                sequence_number BIGINT NOT NULL,
                data TEXT NOT NULL,
                PRIMARY KEY(entity_id, sequence_number)
            )",
            &[],
        )?;

        Ok(Self { client, _marker: Default::default() })
    }
}

impl<E: Serialize + DeserializeOwned> EventsJournal<E> for PostgresEventStore<E> {
    fn persist_event(&mut self, entity_id: i64, seq_event: &SequencedEvent<E>) -> Result<(), &'static str> {
        let serialized_event = serde_json::to_string(&seq_event.event).map_err(|_| "Failed to serialize event")?;
        self.client.execute(
            "INSERT INTO events (entity_id, sequence_number, data) VALUES ($1, $2, $3)",
            &[&entity_id, &seq_event.sequence_number, &serialized_event],
        ).map_err(|_| "Failed to persist event")?;
        Ok(())
    }
    fn retrieve_events(&mut self, entity_id: i64) -> Result<Vec<SequencedEvent<E>>, &'static str> {
        let rows = self.client
            .query("SELECT sequence_number, data FROM events WHERE entity_id = $1 ORDER BY sequence_number ASC", &[&entity_id])
            .map_err(|_| "Failed to retrieve events")?;

        rows.iter()
            .map(|row| {
                let sequence_number: i64 = row.get(0);
                let event_data: String = row.get(1);
                let event: E = serde_json::from_str(&event_data).map_err(|_| "Failed to deserialize event")?;

                Ok(SequencedEvent {
                    sequence_number,
                    event,
                })
            })
            .collect()
    }
}
