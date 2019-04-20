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
    use crate::bank::account::prelude::{
        BankAccountAggregate, BankAccountEvent, BankAccountState, WithdrawMoney,
    };
    use eventsourcing::Aggregate;

    #[test]
    fn withdrawing_money_works() {
        // Arrange
        let mut agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000), Vec::new());
        agg.apply(BankAccountEvent::credited(123, 50)).unwrap();
        let cmd = WithdrawMoney::new(123, 49);
        let expected_events = vec![BankAccountEvent::debited(123, 49)];

        // Act
        let result = agg.execute(cmd).unwrap();

        // Assert
        assert_eq!(expected_events, result);
    }

    #[test]
    fn not_enough_funds() {
        // Arrange
        let mut agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000), Vec::new());
        agg.apply(BankAccountEvent::credited(123, 48)).unwrap();
        let cmd = WithdrawMoney::new(123, 49);
        let expected_events = vec![BankAccountEvent::not_enough_funds(123, 49, 48)];

        // Act
        let result = agg.execute(cmd).unwrap();

        // Assert
        assert_eq!(expected_events, result);
    }

}
