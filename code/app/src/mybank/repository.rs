use crate::mybank::event::BankAccountEvent;
use std::sync::Mutex;
use std::{error::Error, fmt};

pub trait BankAccountRepository {
    fn get_events(&self) -> Result<Vec<BankAccountEvent>, BankAccountRepositoryError>;
    fn save_events(&self, events: Vec<BankAccountEvent>) -> Result<(), BankAccountRepositoryError>;
}

pub struct InMemoryBankAccountRepository {
    pub events: Mutex<Vec<BankAccountEvent>>,
}

impl InMemoryBankAccountRepository {
    pub fn new() -> InMemoryBankAccountRepository {
        InMemoryBankAccountRepository {
            events: Mutex::new(Vec::new()),
        }
    }
}

impl BankAccountRepository for InMemoryBankAccountRepository {
    fn save_events(&self, events: Vec<BankAccountEvent>) -> Result<(), BankAccountRepositoryError> {
        let mut m_entities = self.events.lock().unwrap();

        for event in events {
            m_entities.push(event.clone());
        }

        Ok(())
    }
    fn get_events(&self) -> Result<Vec<BankAccountEvent>, BankAccountRepositoryError> {
        let m_entities = self.events.lock().unwrap();
        let mut values = Vec::new();
        for value in m_entities.iter() {
            values.push((*value).clone());
        }

        Ok(values)
    }
}

#[derive(Debug, PartialEq)]
pub enum BankAccountRepositoryError {
    Unexpected,
}

impl Error for BankAccountRepositoryError {}

impl fmt::Display for BankAccountRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BankAccountRepositoryError: :(")
    }
}
