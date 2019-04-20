mod close_bank_account;
mod deposit_money;
mod errors;
mod events;
mod open_bank_account;
pub mod prelude;
mod types;
mod withdraw_money;

use crate::bank::account::prelude::BankAccountEvent;
use crate::bank::account::types::{BankAccountId, CustomerId};
use eventsourcing::Aggregate;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BankAccountState {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
    pub balance: u64,
    pub generation: u64,
}

impl BankAccountState {
    pub fn new(id: BankAccountId, customer_id: CustomerId) -> BankAccountState {
        BankAccountState {
            id: id,
            customer_id: customer_id,
            balance: 0,
            generation: 0,
        }
    }
}

type NewEvents = Vec<BankAccountEvent>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BankAccountAggregate {
    Opened(BankAccountState, NewEvents),
    Closed(BankAccountState, NewEvents),
    Uninitialized,
}

impl Default for BankAccountAggregate {
    fn default() -> Self {
        BankAccountAggregate::Uninitialized
    }
}

impl Aggregate for BankAccountAggregate {
    fn aggregate_type() -> &'static str
    where
        Self: Sized,
    {
        "BankAccount"
    }

    fn increment_generation(&mut self) {
        use BankAccountAggregate::*;

        match self {
            Opened(data, _) => data.generation += 1,
            Closed(data, _) => data.generation += 1,
            Uninitialized => panic!("CANT INCREMENT GENERATION ON UNINITIALIZED BANK ACC"),
        }
    }
}

impl BankAccountAggregate {
    pub fn open(&mut self, id: BankAccountId, customer_id: CustomerId) -> Result<(), ()> {
        let event = BankAccountEvent::opened(id, customer_id);
        self.record(event).unwrap();
        Ok(())
    }

    pub fn record(&mut self, event: BankAccountEvent) -> Result<(), ()> {
        self.apply(event.clone()).unwrap();

        use BankAccountAggregate::*;
        match self {
            Opened(_, x) => x.push(event),
            Closed(_, x) => x.push(event),
            Uninitialized => panic!("CANT RECORD ON UNINITIALIZED BANK ACC"),
        }

        Ok(())
    }

    pub fn get_new_events(&self) -> Vec<BankAccountEvent> {
        use BankAccountAggregate::*;
        match self {
            Opened(_, x) => x.to_owned(),
            Closed(_, x) => x.to_owned(),
            Uninitialized => panic!("NO EVENTS ON UNINITIALIZED BANK ACC"),
        }
    }
}
