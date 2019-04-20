pub mod eventstore;

use std::fmt;

pub trait Aggregate: Default {
    fn aggregate_type() -> &'static str;
    fn increment_generation(&mut self);

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
        let result = event.apply_to(self);
        self.increment_generation();
        result
    }
}

pub trait Event {
    fn event_type(&self) -> &'static str;
}

pub trait AggregateEvent<A: Aggregate>: Event {
    type Error: CqrsError;
    fn apply_to(self, aggregate: &mut A) -> Result<(), Self::Error>;
}

pub trait AggregateCommand<A: Aggregate> {
    type Event: AggregateEvent<A>;
    type Events: Events<ProducedEvent<A, Self>>;
    type Error: CqrsError;
    fn execute_on(self, aggregate: &A) -> Result<Self::Events, Self::Error>;
}

pub type ProducedEvent<A, C> = <C as AggregateCommand<A>>::Event;

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

pub trait CqrsError: fmt::Debug + fmt::Display + Send + Sync + 'static {}

impl<T> CqrsError for T where T: fmt::Debug + fmt::Display + Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
