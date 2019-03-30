use super::prelude::*;
use std::sync::Arc;

pub struct WithdrawHandler<T>
where
    T: BankAccountRepository,
{
    pub repository: Arc<T>,
}

impl<T: BankAccountRepository> WithdrawHandler<T> {
    pub fn handle(&self, command: WithdrawPayload) -> Result<(), Error> {
        let repo = Arc::clone(&self.repository);

        let current_events = repo.get_events();
        let initial_state = BankAccountAggregate::apply_events(current_events.unwrap());

        let result: Result<Vec<BankAccountEvent>, Error> =
            BankAccountAggregate::withdraw(initial_state.unwrap(), command);

        let events = result?;

        match repo.save_events(events) {
            Ok(()) => Ok(()),
            _ => Err(Error::CantSaveEvent),
        }
    }
}
