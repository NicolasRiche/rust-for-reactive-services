#[derive(Debug)]
pub struct SequencedEvent<E> {
    pub sequence_number: u64,
    pub event: E,
}

/// Define the entry point for interactions with an Entity
pub trait AggregateRoot {
    type State;
    type Command;
    type Error;
    type Event;

    /// Create the entity from historical events.
    /// Return the read only state of the entity after applying the events.
    fn from_events(&mut self, events: Vec<SequencedEvent<Self::Event>>)
        -> Result<&Self::State, Self::Error>;

    /// Handle a command
    /// Success: Return the updated read only state + the sequence of applied events.
    /// Failure: Return an error, the state is unchanged. 
    fn handle_command(&mut self, command: Self::Command) 
        -> Result<(&Self::State, Vec<SequencedEvent<Self::Event>>), Self::Error>;
}
