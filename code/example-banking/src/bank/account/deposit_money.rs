use super::errors::CommandError;
use super::types::BankAccountId;
use super::BankAccountAggregate;
use crate::bank::account::events::BankAccountEvent;
use eventsourcing::AggregateCommand;

/// Create a new to-do item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepositMoney {
    pub id: BankAccountId,
    pub amount: u64,
}

impl DepositMoney {
    pub fn new(id: BankAccountId, amount: u64) -> DepositMoney {
        DepositMoney { id, amount }
    }
}

impl AggregateCommand<BankAccountAggregate> for DepositMoney {
    type Error = CommandError;
    type Event = BankAccountEvent;
    type Events = Vec<Self::Event>;

    fn execute_on(self, aggregate: &BankAccountAggregate) -> Result<Self::Events, Self::Error> {
        if let BankAccountAggregate::Opened(ref _data) = aggregate {
            let events = vec![BankAccountEvent::credited(self.id, self.amount)];
            Ok(events)
        } else {
            Err(CommandError::NotOpened)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bank::account::prelude::{
        BankAccountAggregate, BankAccountEvent, BankAccountState, DepositMoney,
    };
    use eventsourcing::Aggregate;

    #[test]
    fn depositing_money_works() {
        // Arrange
        let agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
        let cmd = DepositMoney::new(123, 49);
        let expected_events = vec![BankAccountEvent::credited(123, 49)];

        // Act
        let result = agg.execute(cmd).unwrap();

        // Assert
        assert_eq!(expected_events, result);
    }

}
