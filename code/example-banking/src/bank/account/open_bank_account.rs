use super::errors::CommandError;
use super::types::{BankAccountId, CustomerId};
use super::BankAccountAggregate;
use crate::bank::account::events::BankAccountEvent;
use eventsourcing::AggregateCommand;

pub struct BankAccountRepository {}

impl BankAccountRepository {
    pub fn save(&self, _events: Vec<BankAccountEvent>) -> Result<(), ()> {
        Ok(())
    }
}

pub struct OpenBankAccountHandler {
    repository: BankAccountRepository,
}

impl OpenBankAccountHandler {
    pub fn new(repository: BankAccountRepository) -> OpenBankAccountHandler {
        OpenBankAccountHandler { repository }
    }

    pub fn handle(&self, cmd: OpenBankAccount) -> Result<(), ()> {
        // Create aggregate
        let mut agg = BankAccountAggregate::default();
        // Get events
        agg.open(cmd.id, cmd.customer_id)?;

        let events = agg.get_new_events();

        if let BankAccountAggregate::Opened(state, zz) = agg {
            println!("{:?}", state);
            println!("{:?}", zz);
        }

        println!("{:?}", &events);

        // Store events
        self.repository.save(events)
    }
}

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
        if let BankAccountAggregate::Opened(_, _) = aggregate {
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
        BankAccountAggregate, BankAccountEvent, BankAccountId, CustomerId, OpenBankAccount,
    };
    use eventsourcing::Aggregate;

    const ACCOUNT_ID: BankAccountId = 123;
    const CUSTOMER_ID: CustomerId = 5000;

    #[test]
    fn open_bank_account_works() {
        assert_open(
            vec![],
            OpenBankAccount::new(ACCOUNT_ID, CUSTOMER_ID),
            Ok(vec![BankAccountEvent::opened(ACCOUNT_ID, CUSTOMER_ID)]),
        );
    }

    #[test]
    fn cant_open_already_opened_bank_account() {
        assert_open(
            vec![BankAccountEvent::opened(ACCOUNT_ID, CUSTOMER_ID)],
            OpenBankAccount::new(ACCOUNT_ID, CUSTOMER_ID),
            Err(CommandError::AlreadyCreated),
        );
    }

    fn assert_open(
        intitial_events: Vec<BankAccountEvent>,
        cmd: OpenBankAccount,
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
