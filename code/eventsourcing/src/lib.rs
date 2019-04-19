use std::fmt;

pub trait Aggregate: Default {
    fn aggregate_type() -> &'static str;

    fn execute<C>(&self, command: C) -> Result<C::Events, C::Error>
    where
        C: AggregateCommand<Self>,
    {
        command.execute_on(self)
    }

    fn apply<E>(&mut self, event: E) -> Result<(), E::Error>
    where
        E: AggregateEvent<Self>,
    {
        event.apply_to(self)
    }
}

pub trait Event {
    fn event_type(&self) -> &'static str;
}

pub trait AggregateEvent<A: Aggregate>: Event {
    /// The error type.
    type Error: CqrsError;
    fn apply_to(self, aggregate: &mut A) -> Result<(), Self::Error>;
}

/// A command that can be executed against an aggregate.
pub trait AggregateCommand<A: Aggregate> {
    /// The type of event that is produced by this command.
    type Event: AggregateEvent<A>;

    /// The type of the sequence of events generated when the command is executed successfully.
    type Events: Events<ProducedEvent<A, Self>>;

    /// The error type.
    type Error: CqrsError;

    /// Consumes a command, attempting to execute it against the aggregate. If the execution is successful, a sequence
    /// of events is generated, which can be applied to the aggregate.
    fn execute_on(self, aggregate: &A) -> Result<Self::Events, Self::Error>;
}

/// The event type produced by this command.
pub type ProducedEvent<A, C> = <C as AggregateCommand<A>>::Event;

/// The event sequence produced by this command.
pub type ProducedEvents<A, C> = <C as AggregateCommand<A>>::Events;

/// The error produced when this command cannot be executed against an aggregate.
pub type CommandError<A, C> = <C as AggregateCommand<A>>::Error;

/// An iterable and sliceable list of events.
pub trait Events<E>: IntoIterator<Item = E> + AsRef<[E]>
where
    E: Event,
{
}

impl<T, E> Events<E> for T
where
    T: IntoIterator<Item = E> + AsRef<[E]>,
    E: Event,
{
}

/// Represents a common trait that all errors handled by CQRS should implement.
pub trait CqrsError: fmt::Debug + fmt::Display + Send + Sync + 'static {}

impl<T> CqrsError for T where T: fmt::Debug + fmt::Display + Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
