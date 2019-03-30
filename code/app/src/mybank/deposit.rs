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

        let current_events = repo.get_events();
        let initial_state = BankAccountAggregate::apply_events(current_events.unwrap());

        let result: Result<Vec<BankAccountEvent>, BankAccountError> =
            BankAccountAggregate::deposit(initial_state.unwrap(), command);

        let events = result?;

        match repo.save_events(events) {
            Ok(()) => Ok(()),
            _ => Err(BankAccountError::CantSaveEvent),
        }
    }
}
