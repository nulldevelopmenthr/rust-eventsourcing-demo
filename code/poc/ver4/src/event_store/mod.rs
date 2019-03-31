use crate::event::BankAccountEvent;
use crate::model::BankAccountError;
use crate::model::BankAccountId;
use std::sync::Mutex;
use std::{error::Error, fmt};

type GetEventsResult = Result<Vec<BankAccountEvent>, BankAccountEventStoreError>;
type SaveEventsResult = Result<(), BankAccountEventStoreError>;

pub trait BankAccountEventStore {
    fn get_events(&self, id: BankAccountId) -> GetEventsResult;
    fn save_events(&self, events: Vec<BankAccountEvent>) -> SaveEventsResult;
}

pub struct InMemoryBankAccountEventStore {
    pub events: Mutex<Vec<BankAccountEvent>>,
}

impl InMemoryBankAccountEventStore {
    pub fn new() -> InMemoryBankAccountEventStore {
        InMemoryBankAccountEventStore {
            events: Mutex::new(Vec::new()),
        }
    }
}

impl BankAccountEventStore for InMemoryBankAccountEventStore {
    fn get_events(&self, id: BankAccountId) -> GetEventsResult {
        let m_entities = self.events.lock().unwrap();
        let mut values = Vec::new();
        for value in m_entities.iter() {
            if id == value.get_aggregate_id() {
                values.push((*value).clone());
            }
        }

        Ok(values)
    }
    fn save_events(&self, events: Vec<BankAccountEvent>) -> SaveEventsResult {
        let mut m_entities = self.events.lock().unwrap();

        for event in events {
            m_entities.push(event.clone());
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum BankAccountEventStoreError {
    TestFailed,
}

impl Error for BankAccountEventStoreError {}

impl fmt::Display for BankAccountEventStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BankAccountEventStoreError: :(")
    }
}

impl From<BankAccountEventStoreError> for BankAccountError {
    fn from(_err: BankAccountEventStoreError) -> BankAccountError {
        BankAccountError::CantSaveEvent
    }
}

#[cfg(test)]
mod tests {
    use crate::event_store::{BankAccountEventStore, InMemoryBankAccountEventStore};
    use crate::prelude::BankAccountEvent;

    #[test]
    fn check_get_events_returns_only_events_with_expected_id() {
        // Arrange
        let event_store = InMemoryBankAccountEventStore::new();

        let events = vec![
            BankAccountEvent::acc_opened(100, 20),
            BankAccountEvent::acc_opened(101, 20),
        ];
        let expected = vec![BankAccountEvent::acc_opened(100, 20)];

        match event_store.save_events(events) {
            Ok(_) => println!("Events saved"),
            Err(_) => panic!("Cant save events"),
        }

        // Act
        let result = event_store.get_events(100).unwrap();

        // Assert
        assert_eq!(expected, result);
    }
}
