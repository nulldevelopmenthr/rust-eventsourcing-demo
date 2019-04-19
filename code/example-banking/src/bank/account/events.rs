use super::types::*;
use super::{BankAccountAggregate, BankAccountState};
use crate::bank::account::errors::EventError;
use eventsourcing::{AggregateEvent, Event};

#[derive(Debug, PartialEq)]
pub enum BankAccountEvent {
    Opened(Opened),
}

impl BankAccountEvent {
    pub fn opened(id: BankAccountId, customer_id: CustomerId) -> BankAccountEvent {
        BankAccountEvent::Opened(Opened { id, customer_id })
    }
}

impl Event for BankAccountEvent {
    fn event_type(&self) -> &'static str {
        match *self {
            BankAccountEvent::Opened(ref evt) => evt.event_type(),
        }
    }
}

impl AggregateEvent<BankAccountAggregate> for BankAccountEvent {
    type Error = EventError;
    fn apply_to(self, aggregate: &mut BankAccountAggregate) -> Result<(), Self::Error> {
        match self {
            BankAccountEvent::Opened(evt) => evt.apply_to(aggregate),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Opened {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
}

impl Event for Opened {
    fn event_type(&self) -> &'static str {
        "opened"
    }
}

impl AggregateEvent<BankAccountAggregate> for Opened {
    type Error = EventError;
    fn apply_to(self, aggregate: &mut BankAccountAggregate) -> Result<(), Self::Error> {
        if BankAccountAggregate::Uninitialized == *aggregate {
            *aggregate =
                BankAccountAggregate::Opened(BankAccountState::new(self.id, self.customer_id));
            Ok(())
        } else {
            Err(EventError::AlreadyOpened)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bank::account::errors::EventError;
    use crate::bank::account::prelude::{BankAccountAggregate, BankAccountEvent, BankAccountState};
    use eventsourcing::Aggregate;

    #[test]
    fn bank_account_opened() {
        // Arrange
        let mut agg = BankAccountAggregate::default();
        let event = BankAccountEvent::opened(123, 5000);
        let expected_agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));

        // Act
        agg.apply(event).unwrap();

        // Assert
        assert_eq!(expected_agg, agg);
    }

    #[test]
    fn throws_error_if_opening_an_opened_account() {
        // Arrange
        let mut agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
        let event = BankAccountEvent::opened(123, 5000);
        let expected_error = Err(EventError::AlreadyOpened);

        // Act
        let result = agg.apply(event);

        // Assert
        assert_eq!(expected_error, result);
    }

}
