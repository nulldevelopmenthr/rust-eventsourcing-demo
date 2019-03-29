use super::command::BankAccountCommand;
use super::command::OpenBankAccountPayload;
use super::event::BankAccountEvent;
use super::event::BankAccountOpened;
use crate::mybank::command::DepositPayload;
use crate::mybank::command::WithdrawPayload;
use crate::mybank::event::BankAccountCredited;
use crate::mybank::event::BankAccountDebited;
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
    pub balance: u64,
}

#[derive(Debug)]
pub struct BankAccountAggregate;

impl BankAccountAggregate {
    pub fn handle(state: MaybeState, command: BankAccountCommand) -> Result<Events, Error> {
        match command {
            BankAccountCommand::OpenBankAccount(payload) => Self::open_acc(payload),
            BankAccountCommand::Deposit(payload) => Self::deposit(state.unwrap(), payload),
            BankAccountCommand::Withdraw(payload) => Self::withdraw(state.unwrap(), payload),
        }
    }

    fn open_acc(input: OpenBankAccountPayload) -> Result<Events, Error> {
        let event = BankAccountEvent::acc_opened(input.id, input.customer_id);
        Ok(vec![event])
    }

    fn deposit(_state: BankAccountState, input: DepositPayload) -> Result<Events, Error> {
        let event = BankAccountEvent::credited(input.id, input.amount);
        Ok(vec![event])
    }

    fn withdraw(state: BankAccountState, input: WithdrawPayload) -> Result<Events, Error> {
        let event = match state.balance >= input.amount {
            true => BankAccountEvent::debited(input.id, input.amount),
            false => BankAccountEvent::withdrawal_refused(input.id, input.amount, state.balance),
        };

        Ok(vec![event])
    }
}

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

    fn apply_event(state: MaybeState, event: &BankAccountEvent) -> Result<MaybeState, Error> {
        let new_state = match event {
            BankAccountEvent::BankAccountOpened(payload) => Self::account_opened(payload),
            BankAccountEvent::Credited(payload) => Self::account_credited(state.unwrap(), payload),
            BankAccountEvent::Debited(payload) => Self::account_debited(state.unwrap(), payload),
            BankAccountEvent::WithdrawalRefused(_payload) => state.unwrap(),
        };

        Ok(Some(new_state))
    }

    fn account_opened(e: &BankAccountOpened) -> BankAccountState {
        BankAccountState {
            id: e.id,
            customer_id: e.customer_id,
            balance: 0,
        }
    }

    fn account_credited(state: BankAccountState, e: &BankAccountCredited) -> BankAccountState {
        BankAccountState {
            id: state.id,
            customer_id: state.customer_id,
            balance: state.balance + e.amount,
        }
    }

    fn account_debited(state: BankAccountState, e: &BankAccountDebited) -> BankAccountState {
        BankAccountState {
            id: state.id,
            customer_id: state.customer_id,
            balance: state.balance - e.amount,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NoState,
}
