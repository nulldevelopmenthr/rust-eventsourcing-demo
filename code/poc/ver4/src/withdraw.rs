use crate::prelude::*;
use std::sync::Arc;

pub struct WithdrawHandler {
    pub repository: Arc<BankAccountRepository>,
}

impl WithdrawHandler {
    pub fn new(repository: Arc<BankAccountRepository>) -> WithdrawHandler {
        WithdrawHandler {
            repository: repository,
        }
    }
    pub fn handle(&self, command: WithdrawMoney) -> Result<(), BankAccountError> {
        let repo = Arc::clone(&self.repository);

        let mut agg = self.repository.load(command.id)?;

        agg.withdraw(command)?;

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
    fn withdraw_handler() {
        // Arrange
        let event_store = Arc::new(TestBankAccountEventStore {});
        let repo = Arc::new(BankAccountRepository::new(event_store));
        let handler = WithdrawHandler::new(repo);

        // Act
        let result = handler.handle(WithdrawMoney::new(100, 7));

        // Assert
        assert_eq!(Ok(()), result);
    }

    #[test]
    fn withdraw_refused_handler() {
        // Arrange
        let event_store = Arc::new(Test2BankAccountEventStore {});
        let repo = BankAccountRepository::new(event_store);
        let handler = WithdrawHandler::new(Arc::new(repo));

        // Act
        let result = handler.handle(WithdrawMoney::new(100, 70));

        // Assert
        assert_eq!(Ok(()), result);
    }

    pub struct TestBankAccountEventStore {}

    pub struct Test2BankAccountEventStore {}

    impl BankAccountEventStore for TestBankAccountEventStore {
        fn save_events(&self, events: Vec<BankAccountEvent>) -> SaveEventsResult {
            let expected = vec![BankAccountEvent::debited(100, 7)];
            match events == expected {
                true => Ok(()),
                false => Err(BankAccountEventStoreError::TestFailed),
            }
        }

        fn get_events(&self, _id: BankAccountId) -> GetEventsResult {
            Ok(vec![
                BankAccountEvent::acc_opened(100, 20),
                BankAccountEvent::credited(100, 49),
            ])
        }
    }

    impl BankAccountEventStore for Test2BankAccountEventStore {
        fn save_events(&self, events: Vec<BankAccountEvent>) -> SaveEventsResult {
            let expected = vec![BankAccountEvent::withdrawal_refused(100, 70, 49)];
            match events == expected {
                true => Ok(()),
                false => Err(BankAccountEventStoreError::TestFailed),
            }
        }

        fn get_events(&self, _id: BankAccountId) -> GetEventsResult {
            Ok(vec![
                BankAccountEvent::acc_opened(100, 20),
                BankAccountEvent::credited(100, 49),
            ])
        }
    }

}
