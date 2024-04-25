use std::collections::HashMap;
use std::marker::PhantomData;
use reactive_service_domain::aggregate_root::SequencedEvent;
use crate::order_service::EventsJournal;

pub struct InMemoryJournal<E> {
    events: HashMap<i64, Vec<SequencedEvent<E>>>,
    _marker: PhantomData<E>,
}

impl <E> InMemoryJournal<E> {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { events: HashMap::default(), _marker: Default::default() })
    }
}

impl<E: Clone> EventsJournal<E> for InMemoryJournal<E> {
    fn persist_event(&mut self, aggregate_id: i64, seq_event: &SequencedEvent<E>) -> Result<(), &'static str> {
        let _ = &self.events.entry(aggregate_id)
            .or_default()
            .push(seq_event.clone());
        Ok(())
    }

    fn retrieve_events(&mut self, aggregate_id: i64) -> Result<Vec<SequencedEvent<E>>, &'static str> {
        let events = &self.events;
        Ok(events.get(&aggregate_id).cloned().unwrap_or_else(Vec::new))
    }
}
