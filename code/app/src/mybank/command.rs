use super::model::{BankAccountId, CustomerId};

//
//     Commands
//
#[derive(Debug)]
pub enum BankAccountCommand {
    OpenBankAccount(OpenBankAccountPayload),
    Deposit(DepositPayload),
    Withdraw(WithdrawPayload),
}

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

impl BankAccountCommand {
    pub fn open_acc(id: BankAccountId, customer_id: CustomerId) -> BankAccountCommand {
        let payload = OpenBankAccountPayload {
            id: id,
            customer_id: customer_id,
        };

        BankAccountCommand::OpenBankAccount(payload)
    }
    pub fn deposit(id: BankAccountId, amount: u64) -> BankAccountCommand {
        let payload = DepositPayload {
            id: id,
            amount: amount,
        };

        BankAccountCommand::Deposit(payload)
    }
    pub fn withdraw(id: BankAccountId, amount: u64) -> BankAccountCommand {
        let payload = WithdrawPayload {
            id: id,
            amount: amount,
        };

        BankAccountCommand::Withdraw(payload)
    }
}
