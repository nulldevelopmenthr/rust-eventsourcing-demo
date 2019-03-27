use super::model::{BankAccountId, CustomerId};

//
//     Events
//
#[derive(Debug, PartialEq)]
pub enum BankAccountEvent {
    BankAccountOpened(BankAccountOpened),
}

#[derive(Debug, PartialEq)]
pub struct BankAccountOpened {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
}

impl BankAccountEvent {
    pub fn acc_opened(id: BankAccountId, customer_id: CustomerId) -> BankAccountEvent {
        BankAccountEvent::BankAccountOpened(BankAccountOpened {
            id: id,
            customer_id: customer_id,
        })
    }
}
