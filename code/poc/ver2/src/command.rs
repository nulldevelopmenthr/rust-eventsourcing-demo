use super::model::{BankAccountId, CustomerId};

//
//     Commands
//
#[derive(Debug)]
pub struct OpenBankAccountPayload {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
}

#[derive(Debug)]
pub struct DepositPayload {
    pub id: BankAccountId,
    pub amount: u64,
}

#[derive(Debug)]
pub struct WithdrawPayload {
    pub id: BankAccountId,
    pub amount: u64,
}
