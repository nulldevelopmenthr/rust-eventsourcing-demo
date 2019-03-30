use super::model::BankAccountId;
use super::prelude::*;
use std::sync::Arc;

pub struct WithdrawHandler<T>
where
    T: BankAccountRepository,
{
    pub repository: Arc<T>,
}

impl<T: BankAccountRepository> WithdrawHandler<T> {
    pub fn handle(&self, command: WithdrawPayload) -> Result<(), BankAccountError> {
        let repo = Arc::clone(&self.repository);

        let mut agg = self.repository.load(command.id)?;

        agg.withdraw(command)?;

        let _result = repo.save(agg)?;

        Ok(())
    }
}

#[test]
fn withdraw_handler() {
    let repo = Arc::new(TestBankAccountRepository {});
    let handler = WithdrawHandler { repository: repo };

    let result = handler.handle(WithdrawPayload { id: 100, amount: 7 });

    assert_eq!(Ok(()), result);
}

#[test]
fn withdraw_refused_handler() {
    let repo = Arc::new(TestBankAccount2Repository {});
    let handler = WithdrawHandler { repository: repo };

    let result = handler.handle(WithdrawPayload {
        id: 100,
        amount: 70,
    });

    assert_eq!(Ok(()), result);
}

impl OpenBankAccountHandler<TestBankAccountRepository> {}
impl OpenBankAccountHandler<TestBankAccount2Repository> {}

pub struct TestBankAccountRepository {}
pub struct TestBankAccount2Repository {}

impl BankAccountRepository for TestBankAccountRepository {
    fn save_events(&self, events: Vec<BankAccountEvent>) -> Result<(), BankAccountRepositoryError> {
        let expected = vec![BankAccountEvent::debited(100, 7)];
        match events == expected {
            true => Ok(()),
            false => Err(BankAccountRepositoryError::Unexpected),
        }
    }

    fn get_events(&self) -> Result<Vec<BankAccountEvent>, BankAccountRepositoryError> {
        Ok(vec![
            BankAccountEvent::acc_opened(100, 20),
            BankAccountEvent::credited(100, 49),
        ])
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

impl BankAccountRepository for TestBankAccount2Repository {
    fn save_events(&self, events: Vec<BankAccountEvent>) -> Result<(), BankAccountRepositoryError> {
        let expected = vec![BankAccountEvent::withdrawal_refused(100, 70, 49)];
        match events == expected {
            true => Ok(()),
            false => Err(BankAccountRepositoryError::Unexpected),
        }
    }

    fn get_events(&self) -> Result<Vec<BankAccountEvent>, BankAccountRepositoryError> {
        Ok(vec![
            BankAccountEvent::acc_opened(100, 20),
            BankAccountEvent::credited(100, 49),
        ])
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
