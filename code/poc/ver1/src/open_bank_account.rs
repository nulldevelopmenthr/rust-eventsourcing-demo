use super::prelude::*;
use std::sync::Arc;

pub struct OpenBankAccountHandler<T>
where
    T: BankAccountRepository,
{
    pub repository: Arc<T>,
}

impl<T: BankAccountRepository> OpenBankAccountHandler<T> {
    pub fn handle(&self, command: OpenBankAccountPayload) -> Result<(), BankAccountError> {
        let result: Result<Vec<BankAccountEvent>, BankAccountError> =
            BankAccountAggregate::open_acc(command);

        let events = result?;

        let repo = Arc::clone(&self.repository);

        let result = repo.save_events(events)?;

        Ok(result)
    }
}

#[test]
fn open_bank_account_handler() {
    let repo = std::sync::Arc::new(TestBankAccountRepository {});
    let handler = OpenBankAccountHandler { repository: repo };

    let result = handler.handle(OpenBankAccountPayload {
        id: 100,
        customer_id: 20,
    });

    assert_eq!(Ok(()), result);
}

impl OpenBankAccountHandler<TestBankAccountRepository> {}

pub struct TestBankAccountRepository {}

impl BankAccountRepository for TestBankAccountRepository {
    fn save_events(&self, events: Vec<BankAccountEvent>) -> Result<(), BankAccountRepositoryError> {
        let expected = vec![BankAccountEvent::acc_opened(100, 20)];
        match events == expected {
            true => Ok(()),
            false => Err(BankAccountRepositoryError::Unexpected),
        }
    }
    fn get_events(&self) -> Result<Vec<BankAccountEvent>, BankAccountRepositoryError> {
        Ok(Vec::new())
    }
}
