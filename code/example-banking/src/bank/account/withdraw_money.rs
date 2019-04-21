use super::errors::CommandError;
use super::types::BankAccountId;
use super::BankAccountAggregate;
use crate::bank::account::events::BankAccountEvent;
use eventsourcing::AggregateCommand;

/// Create a new to-do item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithdrawMoney {
    pub id: BankAccountId,
    pub amount: u64,
}

impl WithdrawMoney {
    pub fn new(id: BankAccountId, amount: u64) -> WithdrawMoney {
        WithdrawMoney { id, amount }
    }
}

impl AggregateCommand<BankAccountAggregate> for WithdrawMoney {
    type Error = CommandError;
    type Event = BankAccountEvent;
    type Events = Vec<Self::Event>;

    fn execute_on(self, aggregate: &BankAccountAggregate) -> Result<Self::Events, Self::Error> {
        if let BankAccountAggregate::Opened(ref data, _) = aggregate {
            if data.balance >= self.amount {
                Ok(vec![BankAccountEvent::debited(self.id, self.amount)])
            } else {
                Ok(vec![BankAccountEvent::not_enough_funds(
                    self.id,
                    self.amount,
                    data.balance,
                )])
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
        BankAccountAggregate, BankAccountEvent, BankAccountId, CustomerId, WithdrawMoney,
    };
    use eventsourcing::Aggregate;

    const ACCOUNT_ID: BankAccountId = 123;
    const CUSTOMER_ID: CustomerId = 5000;

    #[test]
    fn withdrawing_money_works() {
        assert_withdraw(
            vec![
                BankAccountEvent::opened(ACCOUNT_ID, CUSTOMER_ID),
                BankAccountEvent::credited(ACCOUNT_ID, 50),
            ],
            WithdrawMoney::new(ACCOUNT_ID, 49),
            Ok(vec![BankAccountEvent::debited(ACCOUNT_ID, 49)]),
        );
    }

    #[test]
    fn not_enough_funds() {
        assert_withdraw(
            vec![
                BankAccountEvent::opened(ACCOUNT_ID, CUSTOMER_ID),
                BankAccountEvent::credited(ACCOUNT_ID, 48),
            ],
            WithdrawMoney::new(ACCOUNT_ID, 49),
            Ok(vec![BankAccountEvent::not_enough_funds(ACCOUNT_ID, 49, 48)]),
        );
    }

    fn assert_withdraw(
        intitial_events: Vec<BankAccountEvent>,
        cmd: WithdrawMoney,
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
