use super::model::BankAccountId;
use super::prelude::*;
use std::sync::Arc;

pub struct DepositHandler<T>
where
    T: BankAccountRepository,
{
    pub repository: Arc<T>,
}

impl<T: BankAccountRepository> DepositHandler<T> {
    pub fn handle(&self, command: DepositPayload) -> Result<(), BankAccountError> {
        let repo = Arc::clone(&self.repository);

        let mut agg = self.repository.load(command.id)?;

        agg.deposit(command)?;

        let _result = repo.save(agg)?;

        Ok(())
    }
}

#[test]
fn deposit_handler() {
    let repo = Arc::new(TestBankAccountRepository {});
    let handler = DepositHandler { repository: repo };

    let result = handler.handle(DepositPayload {
        id: 100,
        amount: 49,
    });

    assert_eq!(Ok(()), result);
}

impl OpenBankAccountHandler<TestBankAccountRepository> {}

pub struct TestBankAccountRepository {}

impl BankAccountRepository for TestBankAccountRepository {
    fn save_events(&self, events: Vec<BankAccountEvent>) -> Result<(), BankAccountRepositoryError> {
        let expected = vec![BankAccountEvent::credited(100, 49)];
        match events == expected {
            true => Ok(()),
            false => Err(BankAccountRepositoryError::Unexpected),
        }
    }

    fn get_events(&self) -> Result<Vec<BankAccountEvent>, BankAccountRepositoryError> {
        Ok(vec![BankAccountEvent::acc_opened(100, 20)])
    }

    fn load(&self, _id: BankAccountId) -> Result<BankAccountAggregate, BankAccountError> {
        let mut z = BankAccountAggregate::new();
        z.apply_events(self.get_events()?)?;

        Ok(z)
    }

    fn save(&self, agg: BankAccountAggregate) -> Result<(), BankAccountError> {
        self.save_events(agg.new_events)?;

        Ok(())
    }
}
