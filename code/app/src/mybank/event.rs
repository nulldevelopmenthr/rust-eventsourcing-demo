use super::model::{BankAccountId, CustomerId};

//
//     Events
//
#[derive(Debug, PartialEq)]
pub enum BankAccountEvent {
    BankAccountOpened(BankAccountOpened),
    Credited(BankAccountCredited),
    Debited(BankAccountDebited),
}

#[derive(Debug, PartialEq)]
pub struct BankAccountOpened {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
}
#[derive(Debug, PartialEq)]
pub struct BankAccountCredited {
    pub id: BankAccountId,
    pub amount: u64,
}
#[derive(Debug, PartialEq)]
pub struct BankAccountDebited {
    pub id: BankAccountId,
    pub amount: u64,
}

impl BankAccountEvent {
    pub fn acc_opened(id: BankAccountId, customer_id: CustomerId) -> BankAccountEvent {
        BankAccountEvent::BankAccountOpened(BankAccountOpened {
            id: id,
            customer_id: customer_id,
        })
    }
    pub fn credited(id: BankAccountId, amount: u64) -> BankAccountEvent {
        BankAccountEvent::Credited(BankAccountCredited {
            id: id,
            amount: amount,
        })
    }
    pub fn debited(id: BankAccountId, amount: u64) -> BankAccountEvent {
        BankAccountEvent::Debited(BankAccountDebited {
            id: id,
            amount: amount,
        })
    }
}
