use super::model::{BankAccountId, CustomerId};

//
//     Commands
//
#[derive(Debug)]
pub enum BankAccountCommand {
    OpenBankAccount(OpenBankAccountPayload),
}

#[derive(Debug)]
pub struct OpenBankAccountPayload {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
}

impl BankAccountCommand {
    pub fn open_acc(id: BankAccountId, customer_id: CustomerId) -> BankAccountCommand {
        let payload = OpenBankAccountPayload {
            id: id,
            customer_id: customer_id,
        };

        BankAccountCommand::OpenBankAccount(payload)
    }
}
