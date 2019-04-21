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
        if let BankAccountAggregate::Opened(ref data, _) = aggregate {
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
    use crate::bank::account::errors::CommandError;
    use crate::bank::account::prelude::{
        BankAccountAggregate, BankAccountEvent, BankAccountId, CloseBankAccount, CustomerId,
    };
    use eventsourcing::Aggregate;

    const ACCOUNT_ID: BankAccountId = 123;
    const CUSTOMER_ID: CustomerId = 5000;

    #[test]
    fn closing_works() {
        assert_close(
            vec![BankAccountEvent::opened(ACCOUNT_ID, CUSTOMER_ID)],
            CloseBankAccount::new(ACCOUNT_ID),
            Ok(vec![BankAccountEvent::closed(ACCOUNT_ID)]),
        );
    }

    #[test]
    fn cant_close_account_that_has_funds() {
        assert_close(
            vec![
                BankAccountEvent::opened(ACCOUNT_ID, CUSTOMER_ID),
                BankAccountEvent::credited(ACCOUNT_ID, 20),
            ],
            CloseBankAccount::new(ACCOUNT_ID),
            Ok(vec![
                BankAccountEvent::closing_failed_due_to_funds_available(ACCOUNT_ID, 20),
            ]),
        );
    }

    fn assert_close(
        intitial_events: Vec<BankAccountEvent>,
        cmd: CloseBankAccount,
        expected: Result<Vec<BankAccountEvent>, CommandError>,
    ) {
        // Arrange
        let mut agg = BankAccountAggregate::default();
        for event in intitial_events {
            agg.apply(event).unwrap();
        }

        // Act
        let result = agg.execute(cmd);

        // Assert
        assert_eq!(expected, result);
    }
}
