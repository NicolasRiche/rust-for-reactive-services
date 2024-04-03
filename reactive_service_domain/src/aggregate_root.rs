#[derive(Debug)]
pub struct SequencedEvent<E> {
    pub sequence_number: u64,
    pub event: E,
}

#[derive(Debug)]
pub struct AggregateCommandResult<'a, S, E>{
    pub state: &'a S,
    pub events: Vec<SequencedEvent<E>>
}

pub trait AggregateRoot {
    type State;
    type Command;
    type Error;
    type Event;

    fn load_from_events(&mut self, events: Vec<SequencedEvent<Self::Event>>) 
        -> Result<&Self::State, Self::Error>;

    fn handle_command(&mut self, command: Self::Command) 
        -> Result<AggregateCommandResult<Self::State, Self::Event>, Self::Error>;
}
