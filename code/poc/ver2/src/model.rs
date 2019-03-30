//use super::command::BankAccountCommand;
use super::command::OpenBankAccountPayload;
use super::event::BankAccountEvent;
use super::event::BankAccountOpened;
//use super::command::DepositPayload;
//use super::command::WithdrawPayload;
//use super::event::BankAccountCredited;
//use super::event::BankAccountDebited;
use super::command::DepositPayload;
use super::command::WithdrawPayload;
use super::event::BankAccountCredited;
use super::event::BankAccountDebited;
use std::{error::Error, fmt};

//
//     Types,models
//

pub type BankAccountId = u64;
pub type CustomerId = u64;

pub type Events = Vec<BankAccountEvent>;
type MaybeState = Option<BankAccountState>;

#[derive(Debug, PartialEq)]
pub struct BankAccountState {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
    pub balance: u64,
}

#[derive(Debug)]
pub struct BankAccountAggregate {
    state: MaybeState,
    pub new_events: Events,
}

impl BankAccountAggregate {
    pub fn open_acc(
        input: OpenBankAccountPayload,
    ) -> Result<BankAccountAggregate, BankAccountError> {
        let event = BankAccountEvent::acc_opened(input.id, input.customer_id);

        let mut z = BankAccountAggregate::new();

        z.record_events(vec![event])?;

        Ok(z)
    }

    pub fn deposit(&mut self, input: DepositPayload) -> Result<(), BankAccountError> {
        let event = BankAccountEvent::credited(input.id, input.amount);

        self.record_events(vec![event])?;

        Ok(())
    }

    pub fn withdraw(&mut self, input: WithdrawPayload) -> Result<(), BankAccountError> {
        let event = match &self.state {
            Some(state) => match state.balance >= input.amount {
                true => BankAccountEvent::debited(input.id, input.amount),
                false => {
                    BankAccountEvent::withdrawal_refused(input.id, input.amount, state.balance)
                }
            },
            _ => panic!("asasasdasd"),
        };

        self.record_events(vec![event])?;

        Ok(())
    }
}

impl BankAccountAggregate {
    pub fn new() -> BankAccountAggregate {
        BankAccountAggregate {
            state: None,
            new_events: Vec::new(),
        }
    }

    pub fn record_events(&mut self, events: Events) -> Result<(), BankAccountError> {
        for event in events {
            self.apply_event(&event)?;
            self.new_events.push(event);
        }
        Ok(())
    }

    pub fn apply_events(&mut self, events: Events) -> Result<(), BankAccountError> {
        for event in events {
            self.apply_event(&event)?;
        }
        Ok(())
    }

    fn apply_event(&mut self, event: &BankAccountEvent) -> Result<(), BankAccountError> {
        let result = match event {
            BankAccountEvent::BankAccountOpened(payload) => self.account_opened(payload),
            BankAccountEvent::Credited(payload) => self.account_credited(payload),
            BankAccountEvent::Debited(payload) => self.account_debited(payload),
            BankAccountEvent::WithdrawalRefused(_payload) => Ok(()),
        };

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(BankAccountError::CantApplyEvent),
        }
    }

    fn account_opened(&mut self, e: &BankAccountOpened) -> Result<(), BankAccountError> {
        self.state = Some(BankAccountState {
            id: e.id,
            customer_id: e.customer_id,
            balance: 0,
        });

        Ok(())
    }

    fn account_credited(&mut self, e: &BankAccountCredited) -> Result<(), BankAccountError> {
        match &mut self.state {
            Some(current_state) => {
                current_state.balance += e.amount;
                Ok(())
            }
            _ => Err(BankAccountError::NoState),
        }
    }

    fn account_debited(&mut self, e: &BankAccountDebited) -> Result<(), BankAccountError> {
        match &mut self.state {
            Some(current_state) => {
                current_state.balance -= e.amount;
                Ok(())
            }
            _ => Err(BankAccountError::NoState),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BankAccountError {
    NoState,
    CantSaveEvent,
    CantApplyEvent,
}

impl Error for BankAccountError {}

impl fmt::Display for BankAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BankAccountError: Oh no, something bad went down")
    }
}
