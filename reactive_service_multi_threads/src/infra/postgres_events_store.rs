use serde::Serialize;
use serde::de::DeserializeOwned;
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use reactive_service_domain::aggregate_root::SequencedEvent;
use crate::order_service::EventsJournal;

pub struct PostgresEventStore { pool: Pool<PostgresConnectionManager<NoTls>>}

impl PostgresEventStore {

    pub fn new(connection_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let manager = PostgresConnectionManager::new(connection_str.parse()?, NoTls);
        let pool = Pool::new(manager)?;

        let mut conn = pool.get()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                entity_id BIGINT NOT NULL,
                sequence_number BIGINT NOT NULL,
                payload TEXT NOT NULL,
                PRIMARY KEY(entity_id, sequence_number)
            )",
            &[],
        )?;

        Ok(Self{pool})
    }
}

impl<E: Serialize + DeserializeOwned> EventsJournal<E> for PostgresEventStore {
    fn persist_event(&self, entity_id: i64, seq_event: &SequencedEvent<E>) -> Result<(), &'static str> {
        let mut conn = self.pool.get().expect("Failed to get a DB connection");
        let serialized_event = serde_json::to_string(&seq_event.event).map_err(|_| "Failed to serialize event")?;
        conn.execute(
            "INSERT INTO events (entity_id, sequence_number, payload) VALUES ($1, $2, $3)",
            &[&entity_id, &seq_event.sequence_number, &serialized_event],
        ).map_err(|_| "Failed to persist event")?;
        Ok(())
    }
    fn retrieve_events(&self, entity_id: i64) -> Result<Vec<SequencedEvent<E>>, &'static str> {
        let mut conn = self.pool.get().expect("Failed to get a DB connection");
        let rows = conn
            .query("SELECT sequence_number, payload FROM events WHERE entity_id = $1 ORDER BY sequence_number ASC", &[&entity_id])
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
