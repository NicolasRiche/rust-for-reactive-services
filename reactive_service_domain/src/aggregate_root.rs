#[derive(Debug, Clone)]
pub struct SequencedEvent<E> {
    pub sequence_number: u64,
    pub event: E,
}

pub trait AggregateRoot {
    type EntityId;
    type Command;
    type Event;
    type State;
    type Error;

    fn handle_command(&mut self, command: Self::Command) -> Result<(&Self::State, Vec<SequencedEvent<Self::Event>>), Self::Error>;

    fn apply_event(&mut self, seq_evt: SequencedEvent<Self::Event>) -> &Self::State;
}
