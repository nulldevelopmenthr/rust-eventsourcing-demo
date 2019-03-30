use super::event::BankAccountEvent;
use super::model::BankAccountAggregate;
use super::model::BankAccountError;
use super::model::BankAccountId;
use std::sync::Mutex;
use std::{error::Error, fmt};

pub trait BankAccountRepository {
    fn get_events(&self) -> Result<Vec<BankAccountEvent>, BankAccountRepositoryError>;
    fn save_events(&self, events: Vec<BankAccountEvent>) -> Result<(), BankAccountRepositoryError>;

    fn load(&self, id: BankAccountId) -> Result<BankAccountAggregate, BankAccountError>;
    fn save(&self, agg: BankAccountAggregate) -> Result<(), BankAccountError>;
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
    fn load(&self, _id: BankAccountId) -> Result<BankAccountAggregate, BankAccountError> {
        let events = self.get_events()?;

        let mut z = BankAccountAggregate::new();

        z.apply_events(events)?;

        Ok(z)
    }

    fn save(&self, agg: BankAccountAggregate) -> Result<(), BankAccountError> {
        self.save_events(agg.new_events)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum BankAccountRepositoryError {
    Unexpected,
    NotImplemented,
}

impl Error for BankAccountRepositoryError {}

impl fmt::Display for BankAccountRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BankAccountRepositoryError: :(")
    }
}

impl From<BankAccountRepositoryError> for BankAccountError {
    fn from(_err: BankAccountRepositoryError) -> BankAccountError {
        BankAccountError::CantSaveEvent
    }
}
