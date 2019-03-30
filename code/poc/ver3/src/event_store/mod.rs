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
    fn get_events(&self, _id: BankAccountId) -> GetEventsResult {
        let m_entities = self.events.lock().unwrap();
        let mut values = Vec::new();
        for value in m_entities.iter() {
            values.push((*value).clone());
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
