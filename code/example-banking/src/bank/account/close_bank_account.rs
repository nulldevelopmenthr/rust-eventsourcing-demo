use super::errors::CommandError;
use super::types::BankAccountId;
use super::BankAccountAggregate;
use crate::bank::account::events::BankAccountEvent;
use eventsourcing::AggregateCommand;

/// Create a new to-do item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseBankAccount {
    pub id: BankAccountId,
}

impl CloseBankAccount {
    pub fn new(id: BankAccountId) -> CloseBankAccount {
        CloseBankAccount { id }
    }
}

impl AggregateCommand<BankAccountAggregate> for CloseBankAccount {
    type Error = CommandError;
    type Event = BankAccountEvent;
    type Events = Vec<Self::Event>;

    fn execute_on(self, aggregate: &BankAccountAggregate) -> Result<Self::Events, Self::Error> {
        if let BankAccountAggregate::Opened(ref data) = aggregate {
            if data.balance == 0 {
                Ok(vec![BankAccountEvent::closed(self.id)])
            } else {
                Ok(vec![
                    BankAccountEvent::closing_failed_due_to_funds_available(self.id, data.balance),
                ])
            }
        } else {
            Err(CommandError::NotOpened)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bank::account::prelude::{
        BankAccountAggregate, BankAccountEvent, BankAccountState, CloseBankAccount,
    };
    use eventsourcing::Aggregate;

    #[test]
    fn closing_works() {
        // Arrange
        let agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
        let cmd = CloseBankAccount::new(123);
        let expected_events = vec![BankAccountEvent::closed(123)];

        // Act
        let result = agg.execute(cmd).unwrap();

        // Assert
        assert_eq!(expected_events, result);
    }

    #[test]
    fn cant_close_account_that_has_funds() {
        // Arrange
        let mut agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
        agg.apply(BankAccountEvent::credited(123, 20)).unwrap();
        let cmd = CloseBankAccount::new(123);
        let expected_events = vec![BankAccountEvent::closing_failed_due_to_funds_available(
            123, 20,
        )];

        // Act
        let result = agg.execute(cmd).unwrap();

        // Assert
        assert_eq!(expected_events, result);
    }
}
