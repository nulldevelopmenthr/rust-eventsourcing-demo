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
        if let BankAccountAggregate::Opened(ref _data, _) = aggregate {
            let events = vec![BankAccountEvent::credited(self.id, self.amount)];
            Ok(events)
        } else {
            Err(CommandError::NotOpened)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bank::account::errors::CommandError;
    use crate::bank::account::prelude::{
        BankAccountAggregate, BankAccountEvent, BankAccountId, CustomerId, DepositMoney,
    };
    use eventsourcing::Aggregate;

    const ACCOUNT_ID: BankAccountId = 123;
    const CUSTOMER_ID: CustomerId = 5000;

    #[test]
    fn depositing_money_works() {
        assert_deposit(
            vec![BankAccountEvent::opened(ACCOUNT_ID, CUSTOMER_ID)],
            DepositMoney::new(ACCOUNT_ID, 49),
            Ok(vec![BankAccountEvent::credited(ACCOUNT_ID, 49)]),
        );
    }

    fn assert_deposit(
        intitial_events: Vec<BankAccountEvent>,
        cmd: DepositMoney,
        expected: Result<Vec<BankAccountEvent>, CommandError>,
    ) {
        //Arrange
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
