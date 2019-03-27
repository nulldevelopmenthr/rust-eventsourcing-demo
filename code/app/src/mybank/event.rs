use super::model::{BankAccountId, CustomerId};

//
//     Events
//
#[derive(Debug, PartialEq)]
pub enum BankAccountEvent {
    BankAccountOpened(BankAccountOpened),
    Credited(BankAccountCredited),
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
}
