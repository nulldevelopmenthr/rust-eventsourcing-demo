use crate::model::{BankAccountId, CustomerId};

//
//     Commands
//
#[derive(Debug)]
pub struct OpenBankAccount {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
}

impl OpenBankAccount {
    pub fn new(id: BankAccountId, customer_id: CustomerId) -> OpenBankAccount {
        OpenBankAccount {
            id: id,
            customer_id: customer_id,
        }
    }
}

#[derive(Debug)]
pub struct DepositMoney {
    pub id: BankAccountId,
    pub amount: u64,
}

impl DepositMoney {
    pub fn new(id: BankAccountId, amount: u64) -> DepositMoney {
        DepositMoney {
            id: id,
            amount: amount,
        }
    }
}

#[derive(Debug)]
pub struct WithdrawMoney {
    pub id: BankAccountId,
    pub amount: u64,
}

impl WithdrawMoney {
    pub fn new(id: BankAccountId, amount: u64) -> WithdrawMoney {
        WithdrawMoney {
            id: id,
            amount: amount,
        }
    }
}
