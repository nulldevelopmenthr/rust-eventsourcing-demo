use crate::prelude::*;
use std::sync::Arc;

pub struct DepositHandler {
    pub repository: Arc<BankAccountRepository>,
}

impl DepositHandler {
    pub fn new(repository: Arc<BankAccountRepository>) -> DepositHandler {
        DepositHandler {
            repository: repository,
        }
    }

    pub fn handle(&self, command: DepositMoney) -> Result<(), BankAccountError> {
        let repo = Arc::clone(&self.repository);

        let mut agg = self.repository.load(command.id)?;

        agg.deposit(command)?;

        let _result = repo.save(agg)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::sync::Arc;

    type GetEventsResult = Result<Vec<BankAccountEvent>, BankAccountEventStoreError>;
    type SaveEventsResult = Result<(), BankAccountEventStoreError>;

    #[test]
    fn deposit_handler() {
        // Arrange
        let event_store = Arc::new(TestBankAccountEventStore {});
        let repo = Arc::new(BankAccountRepository::new(event_store));
        let handler = DepositHandler::new(repo);

        // Act
        let result = handler.handle(DepositMoney::new(100, 49));

        // Assert
        assert_eq!(Ok(()), result);
    }

    pub struct TestBankAccountEventStore {}

    impl BankAccountEventStore for TestBankAccountEventStore {
        fn save_events(&self, events: Vec<BankAccountEvent>) -> SaveEventsResult {
            let expected = vec![BankAccountEvent::credited(100, 49)];
            match events == expected {
                true => Ok(()),
                false => Err(BankAccountEventStoreError::TestFailed),
            }
        }

        fn get_events(&self, _id: BankAccountId) -> GetEventsResult {
            Ok(vec![BankAccountEvent::acc_opened(100, 20)])
        }
    }
}
