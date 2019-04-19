use super::errors::CommandError;
use super::types::{BankAccountId, CustomerId};
use super::BankAccountAggregate;
use crate::bank::account::events::BankAccountEvent;
use eventsourcing::AggregateCommand;

/// Create a new to-do item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenBankAccount {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
}

impl OpenBankAccount {
    pub fn new(id: BankAccountId, customer_id: CustomerId) -> OpenBankAccount {
        OpenBankAccount { id, customer_id }
    }
}

impl AggregateCommand<BankAccountAggregate> for OpenBankAccount {
    type Error = CommandError;
    type Event = BankAccountEvent;
    type Events = Vec<Self::Event>;

    fn execute_on(self, aggregate: &BankAccountAggregate) -> Result<Self::Events, Self::Error> {
        if let BankAccountAggregate::Opened(_) = aggregate {
            return Err(CommandError::AlreadyCreated);
        }

        let events = vec![BankAccountEvent::opened(self.id, self.customer_id)];
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use crate::bank::account::errors::CommandError;
    use crate::bank::account::prelude::{
        BankAccountAggregate, BankAccountEvent, BankAccountState, OpenBankAccount,
    };
    use eventsourcing::Aggregate;

    #[test]
    fn open_bank_account_works() {
        // Arrange
        let agg = BankAccountAggregate::default();
        let cmd = OpenBankAccount::new(123, 5000);
        let expected_events = vec![BankAccountEvent::opened(123, 5000)];

        // Act
        let result = agg.execute(cmd).unwrap();

        // Assert
        assert_eq!(expected_events, result);
    }

    #[test]
    fn cant_open_already_opened_bank_account() {
        // Arrange
        let agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
        let cmd = OpenBankAccount::new(123, 5000);
        let expected_error = Err(CommandError::AlreadyCreated);

        // Act
        let result = agg.execute(cmd);

        // Assert
        assert_eq!(expected_error, result);
    }
}
