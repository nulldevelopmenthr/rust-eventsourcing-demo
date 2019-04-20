mod close_bank_account;
mod deposit_money;
mod errors;
mod events;
mod open_bank_account;
pub mod prelude;
mod types;
mod withdraw_money;
use eventsourcing::Aggregate;
use types::{BankAccountId, CustomerId};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BankAccountState {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
    pub balance: u64,
}

impl BankAccountState {
    pub fn new(id: BankAccountId, customer_id: CustomerId) -> BankAccountState {
        BankAccountState {
            id: id,
            customer_id: customer_id,
            balance: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BankAccountAggregate {
    Opened(BankAccountState),
    Closed(BankAccountState),
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
}
