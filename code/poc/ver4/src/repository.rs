use crate::event_store::BankAccountEventStore;
use crate::event_store::InMemoryBankAccountEventStore;
use crate::model::BankAccountAggregate;
use crate::model::BankAccountError;
use crate::model::BankAccountId;
use std::sync::Arc;
use std::{error::Error, fmt};

pub struct BankAccountRepository {
    pub event_store: Arc<BankAccountEventStore>,
}

impl BankAccountRepository {
    pub fn new(event_store: Arc<BankAccountEventStore>) -> BankAccountRepository {
        BankAccountRepository {
            event_store: event_store,
        }
    }
    pub fn new_in_memory() -> BankAccountRepository {
        BankAccountRepository {
            event_store: Arc::new(InMemoryBankAccountEventStore::new()),
        }
    }
    pub fn load(&self, id: BankAccountId) -> Result<BankAccountAggregate, BankAccountError> {
        let events = self.event_store.get_events(id)?;

        let mut aggregate = BankAccountAggregate::new();

        aggregate.apply_events(events)?;

        Ok(aggregate)
    }

    pub fn save(&self, agg: BankAccountAggregate) -> Result<(), BankAccountError> {
        self.event_store.save_events(agg.new_events)?;

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
