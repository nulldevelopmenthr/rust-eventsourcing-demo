use crate::prelude::*;
use std::sync::Arc;

pub struct OpenBankAccountHandler {
    pub repository: Arc<BankAccountRepository>,
}

impl OpenBankAccountHandler {
    pub fn new(repository: Arc<BankAccountRepository>) -> OpenBankAccountHandler {
        OpenBankAccountHandler {
            repository: repository,
        }
    }
    pub fn handle(&self, command: OpenBankAccount) -> Result<(), BankAccountError> {
        let agg: BankAccountAggregate = BankAccountAggregate::open_acc(command)?;

        let repo = Arc::clone(&self.repository);

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
    fn open_bank_account_handler() {
        // Arrange
        let event_store = Arc::new(TestBankAccountEventStore {});
        let repo = Arc::new(BankAccountRepository::new(event_store));
        let handler = OpenBankAccountHandler::new(repo);

        // Act
        let result = handler.handle(OpenBankAccount::new(100, 20));

        // Assert
        assert_eq!(Ok(()), result);
    }

    pub struct TestBankAccountEventStore {}

    impl BankAccountEventStore for TestBankAccountEventStore {
        fn save_events(&self, events: Vec<BankAccountEvent>) -> SaveEventsResult {
            let expected = vec![BankAccountEvent::acc_opened(100, 20)];
            match events == expected {
                true => Ok(()),
                false => Err(BankAccountEventStoreError::TestFailed),
            }
        }

        fn get_events(&self, _id: BankAccountId) -> GetEventsResult {
            unimplemented!()
        }
    }

}
