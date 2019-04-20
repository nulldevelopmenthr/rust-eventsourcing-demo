pub trait EventStore {}

pub struct DummyEventStore {}

impl EventStore for DummyEventStore {}
