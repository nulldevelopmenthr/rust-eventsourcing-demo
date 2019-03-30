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
        let agg: BankAccountAggregate = BankAccountAggregate::open_acc(command)?;

        let repo = Arc::clone(&self.repository);

        let _result = repo.save(agg)?;

        Ok(())
    }
}

#[test]
fn open_bank_account_handler() {
    let repo = Arc::new(TestBankAccountRepository {});
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
        unimplemented!()
    }

    fn load(&self, _id: BankAccountId) -> Result<BankAccountAggregate, BankAccountError> {
        unimplemented!()
    }

    fn save(&self, agg: BankAccountAggregate) -> Result<(), BankAccountError> {
        self.save_events(agg.new_events)?;

        Ok(())
    }
}
