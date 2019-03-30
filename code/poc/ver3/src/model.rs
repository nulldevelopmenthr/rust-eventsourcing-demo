use crate::command::{DepositMoney, OpenBankAccount, WithdrawMoney};
use crate::event::{BankAccountCredited, BankAccountDebited, BankAccountEvent, BankAccountOpened};
use std::{error::Error, fmt};

//
//     Types,models
//

pub type BankAccountId = u64;
pub type CustomerId = u64;

type Events = Vec<BankAccountEvent>;
type MaybeState = Option<BankAccountState>;
type FactoryResult = Result<BankAccountAggregate, BankAccountError>;
type OkOrError = Result<(), BankAccountError>;

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
    pub fn new() -> BankAccountAggregate {
        BankAccountAggregate {
            state: None,
            new_events: Vec::new(),
        }
    }
}

// This should be probably moved to 'generic' Aggregate
impl BankAccountAggregate {
    pub fn record_events(&mut self, events: Events) -> OkOrError {
        for event in events {
            self.record_event(event)?;
        }
        Ok(())
    }

    pub fn record_event(&mut self, event: BankAccountEvent) -> OkOrError {
        self.apply_event(&event)?;
        self.new_events.push(event);
        Ok(())
    }

    pub fn apply_events(&mut self, events: Events) -> OkOrError {
        for event in events {
            self.apply_event(&event)?;
        }
        Ok(())
    }
}

impl BankAccountAggregate {
    pub fn open_acc(input: OpenBankAccount) -> FactoryResult {
        let event = BankAccountEvent::acc_opened(input.id, input.customer_id);

        let mut aggregate = BankAccountAggregate::new();
        aggregate.record_event(event)?;

        Ok(aggregate)
    }

    pub fn deposit(&mut self, input: DepositMoney) -> OkOrError {
        let event = BankAccountEvent::credited(input.id, input.amount);

        self.record_event(event)?;

        Ok(())
    }

    pub fn withdraw(&mut self, input: WithdrawMoney) -> OkOrError {
        if let Some(state) = &mut self.state {
            let event = match state.balance >= input.amount {
                true => BankAccountEvent::debited(input.id, input.amount),
                false => {
                    BankAccountEvent::withdrawal_refused(input.id, input.amount, state.balance)
                }
            };

            self.record_event(event)?;

            Ok(())
        } else {
            Err(BankAccountError::NoState)
        }
    }
}

impl BankAccountAggregate {
    fn apply_event(&mut self, event: &BankAccountEvent) -> OkOrError {
        match event {
            BankAccountEvent::BankAccountOpened(payload) => self.account_opened(payload),
            BankAccountEvent::Credited(payload) => self.account_credited(payload),
            BankAccountEvent::Debited(payload) => self.account_debited(payload),
            BankAccountEvent::WithdrawalRefused(_payload) => Ok(()),
        }
    }

    fn account_opened(&mut self, e: &BankAccountOpened) -> OkOrError {
        self.state = Some(BankAccountState {
            id: e.id,
            customer_id: e.customer_id,
            balance: 0,
        });

        Ok(())
    }

    fn account_credited(&mut self, e: &BankAccountCredited) -> OkOrError {
        if let Some(current_state) = &mut self.state {
            current_state.balance += e.amount;
            Ok(())
        } else {
            Err(BankAccountError::NoState)
        }
    }

    fn account_debited(&mut self, e: &BankAccountDebited) -> OkOrError {
        if let Some(current_state) = &mut self.state {
            current_state.balance -= e.amount;
            Ok(())
        } else {
            Err(BankAccountError::NoState)
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
