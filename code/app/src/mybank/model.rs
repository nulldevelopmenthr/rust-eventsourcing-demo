use super::command::BankAccountCommand;
use super::command::OpenBankAccountPayload;
use super::event::BankAccountEvent;
use super::event::BankAccountOpened;
//
//     Types,models
//

pub type BankAccountId = u64;
pub type CustomerId = u64;

type Events = Vec<BankAccountEvent>;
type MaybeState = Option<BankAccountState>;

#[derive(Debug, PartialEq)]
pub struct BankAccountState {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
}

#[derive(Debug)]
pub struct BankAccountAggregate;

impl BankAccountAggregate {
    pub fn handle(command: BankAccountCommand) -> Result<Events, Error> {
        match command {
            BankAccountCommand::OpenBankAccount(payload) => Self::open_acc(payload),
        }
    }

    fn open_acc(input: OpenBankAccountPayload) -> Result<Events, Error> {
        let event = BankAccountEvent::acc_opened(input.id, input.customer_id);
        Ok(vec![event])
    }
}

#[allow(dead_code)]
impl BankAccountAggregate {
    pub fn apply_events(events: Events) -> Result<BankAccountState, Error> {
        let mut state = None;

        for event in events {
            let result = Self::apply_event(state, &event);
            state = result.unwrap();
        }

        match state {
            Some(state) => Ok(state),
            None => Err(Error::NoState),
        }
    }

    fn apply_event(_state: MaybeState, event: &BankAccountEvent) -> Result<MaybeState, Error> {
        let state = match event {
            BankAccountEvent::BankAccountOpened(payload) => Self::account_opened(payload),
        };

        Ok(Some(state))
    }

    fn account_opened(e: &BankAccountOpened) -> BankAccountState {
        BankAccountState {
            id: e.id,
            customer_id: e.customer_id,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NoState,
}
