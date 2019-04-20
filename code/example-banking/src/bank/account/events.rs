use super::types::*;
use super::{BankAccountAggregate, BankAccountState};
use crate::bank::account::errors::EventError;
use eventsourcing::{AggregateEvent, Event};

#[derive(Debug, PartialEq)]
pub enum BankAccountEvent {
    Opened(Opened),
    Credited(Credited),
    Debited(Debited),
    NotEnoughFunds(NotEnoughFunds),
    Closed(Closed),
    ClosingFailedDueToFundsAvailable(ClosingFailedDueToFundsAvailable),
}

impl BankAccountEvent {
    pub fn opened(id: BankAccountId, customer_id: CustomerId) -> BankAccountEvent {
        BankAccountEvent::Opened(Opened { id, customer_id })
    }
    pub fn credited(id: BankAccountId, amount: u64) -> BankAccountEvent {
        BankAccountEvent::Credited(Credited { id, amount })
    }
    pub fn debited(id: BankAccountId, amount: u64) -> BankAccountEvent {
        BankAccountEvent::Debited(Debited { id, amount })
    }
    pub fn not_enough_funds(
        id: BankAccountId,
        amount: u64,
        current_balance: u64,
    ) -> BankAccountEvent {
        BankAccountEvent::NotEnoughFunds(NotEnoughFunds {
            id,
            amount,
            current_balance,
        })
    }
    pub fn closed(id: BankAccountId) -> BankAccountEvent {
        BankAccountEvent::Closed(Closed { id })
    }
    pub fn closing_failed_due_to_funds_available(
        id: BankAccountId,
        current_balance: u64,
    ) -> BankAccountEvent {
        BankAccountEvent::ClosingFailedDueToFundsAvailable(ClosingFailedDueToFundsAvailable {
            id,
            current_balance,
        })
    }
}

impl Event for BankAccountEvent {
    fn event_type(&self) -> &'static str {
        match *self {
            BankAccountEvent::Opened(ref evt) => evt.event_type(),
            BankAccountEvent::Credited(ref evt) => evt.event_type(),
            BankAccountEvent::Debited(ref evt) => evt.event_type(),
            BankAccountEvent::NotEnoughFunds(ref evt) => evt.event_type(),
            BankAccountEvent::Closed(ref evt) => evt.event_type(),
            BankAccountEvent::ClosingFailedDueToFundsAvailable(ref evt) => evt.event_type(),
        }
    }
}

impl AggregateEvent<BankAccountAggregate> for BankAccountEvent {
    type Error = EventError;
    fn apply_to(self, aggregate: &mut BankAccountAggregate) -> Result<(), Self::Error> {
        match self {
            BankAccountEvent::Opened(evt) => evt.apply_to(aggregate),
            BankAccountEvent::Credited(evt) => evt.apply_to(aggregate),
            BankAccountEvent::Debited(evt) => evt.apply_to(aggregate),
            BankAccountEvent::NotEnoughFunds(evt) => evt.apply_to(aggregate),
            BankAccountEvent::Closed(evt) => evt.apply_to(aggregate),
            BankAccountEvent::ClosingFailedDueToFundsAvailable(evt) => evt.apply_to(aggregate),
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

#[derive(Debug, PartialEq)]
pub struct Credited {
    pub id: BankAccountId,
    pub amount: u64,
}

impl Event for Credited {
    fn event_type(&self) -> &'static str {
        "credited"
    }
}

impl AggregateEvent<BankAccountAggregate> for Credited {
    type Error = EventError;
    fn apply_to(self, aggregate: &mut BankAccountAggregate) -> Result<(), Self::Error> {
        if let BankAccountAggregate::Opened(ref mut data) = aggregate {
            data.balance += self.amount;
            Ok(())
        } else {
            Err(EventError::NotInitialized)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Debited {
    pub id: BankAccountId,
    pub amount: u64,
}

impl Event for Debited {
    fn event_type(&self) -> &'static str {
        "debited"
    }
}

impl AggregateEvent<BankAccountAggregate> for Debited {
    type Error = EventError;
    fn apply_to(self, aggregate: &mut BankAccountAggregate) -> Result<(), Self::Error> {
        if let BankAccountAggregate::Opened(ref mut data) = aggregate {
            data.balance -= self.amount;
            Ok(())
        } else {
            Err(EventError::NotInitialized)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NotEnoughFunds {
    pub id: BankAccountId,
    pub amount: u64,
    pub current_balance: u64,
}

impl Event for NotEnoughFunds {
    fn event_type(&self) -> &'static str {
        "not_enough_funds"
    }
}

impl AggregateEvent<BankAccountAggregate> for NotEnoughFunds {
    type Error = EventError;
    fn apply_to(self, aggregate: &mut BankAccountAggregate) -> Result<(), Self::Error> {
        if let BankAccountAggregate::Opened(ref mut _data) = aggregate {
            Ok(())
        } else {
            Err(EventError::NotInitialized)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Closed {
    pub id: BankAccountId,
}

impl Event for Closed {
    fn event_type(&self) -> &'static str {
        "closed"
    }
}

impl AggregateEvent<BankAccountAggregate> for Closed {
    type Error = EventError;
    fn apply_to(self, aggregate: &mut BankAccountAggregate) -> Result<(), Self::Error> {
        if let BankAccountAggregate::Opened(ref data) = aggregate {
            *aggregate = BankAccountAggregate::Closed(data.to_owned());
            Ok(())
        } else {
            Err(EventError::AlreadyOpened)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ClosingFailedDueToFundsAvailable {
    pub id: BankAccountId,
    pub current_balance: u64,
}

impl Event for ClosingFailedDueToFundsAvailable {
    fn event_type(&self) -> &'static str {
        "closing_failed_due_to_funds_available"
    }
}

impl AggregateEvent<BankAccountAggregate> for ClosingFailedDueToFundsAvailable {
    type Error = EventError;
    fn apply_to(self, aggregate: &mut BankAccountAggregate) -> Result<(), Self::Error> {
        if let BankAccountAggregate::Opened(ref mut _data) = aggregate {
            Ok(())
        } else {
            Err(EventError::NotOpened)
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

    #[test]
    fn bank_account_credited() {
        // Arrange
        let mut agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
        let event = BankAccountEvent::credited(123, 49);
        let expected_balance = 49;

        // Act
        agg.apply(event).unwrap();

        // Assert
        if let BankAccountAggregate::Opened(state) = agg {
            assert_eq!(expected_balance, state.balance);
        } else {
            panic!("Aggregate not in Opened state");
        }
    }

    #[test]
    fn bank_account_debited() {
        // Arrange
        let mut agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
        let events = vec![
            BankAccountEvent::credited(123, 49),
            BankAccountEvent::debited(123, 48),
        ];
        let expected_balance = 1;

        // Act
        for event in events {
            agg.apply(event).unwrap();
        }

        // Assert
        if let BankAccountAggregate::Opened(state) = agg {
            assert_eq!(expected_balance, state.balance);
        } else {
            panic!("Aggregate not in Opened state");
        }
    }

    #[test]
    fn bank_account_not_enough_funds() {
        // Arrange
        let mut agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
        let event = BankAccountEvent::not_enough_funds(123, 49, 0);
        let expected_balance = 0;

        // Act
        agg.apply(event).unwrap();

        // Assert
        if let BankAccountAggregate::Opened(state) = agg {
            assert_eq!(expected_balance, state.balance);
        } else {
            panic!("Aggregate not in Opened state");
        }
    }

    #[test]
    fn closing_bank_account() {
        // Arrange
        let mut agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
        let event = BankAccountEvent::closed(123);
        let expected_balance = 0;

        // Act
        agg.apply(event).unwrap();

        // Assert
        if let BankAccountAggregate::Closed(state) = agg {
            assert_eq!(expected_balance, state.balance);
        } else {
            panic!("Aggregate not in Closed state");
        }
    }

    #[test]
    fn closing_not_possible_due_to_funds_available() {
        // Arrange
        let mut agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
        let events = vec![
            BankAccountEvent::credited(123, 49),
            BankAccountEvent::closing_failed_due_to_funds_available(123, 49),
        ];
        let expected_balance = 49;

        // Act
        for event in events {
            agg.apply(event).unwrap();
        }

        // Assert
        if let BankAccountAggregate::Opened(state) = agg {
            assert_eq!(expected_balance, state.balance);
        } else {
            panic!("Aggregate not in Opened state");
        }
    }
}
