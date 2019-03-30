use crate::command::DepositPayload;
use crate::command::OpenBankAccountPayload;
use crate::command::WithdrawPayload;
use crate::deposit::DepositHandler;
use crate::event::BankAccountEvent;
use crate::open_bank_account::OpenBankAccountHandler;
use crate::repository::BankAccountRepository;
use crate::repository::InMemoryBankAccountRepository;
use crate::withdraw::WithdrawHandler;
use std::sync::Arc;

pub fn examples() {
    example_open_bank_account();
    example_deposit_money();
    example_withdraw_money();

    println!("Done!");
}

fn example_open_bank_account() {
    let repo = Arc::new(InMemoryBankAccountRepository::new());
    let repo2 = repo.clone();
    let handler = OpenBankAccountHandler { repository: repo };

    let result = handler.handle(OpenBankAccountPayload {
        id: 100,
        customer_id: 20,
    });

    println!("{:?}", &repo2.get_events());

    match result {
        Ok(()) => println!("Bank account opened"),
        _ => panic!("Opening bank account failed"),
    }
}

fn example_deposit_money() {
    let initial_events = vec![BankAccountEvent::acc_opened(100, 20)];
    let repo = Arc::new(InMemoryBankAccountRepository::new());

    match repo.save_events(initial_events) {
        Ok(()) => println!("Setup OK"),
        _ => panic!("Setup failed"),
    }

    let repo2 = repo.clone();
    let handler = DepositHandler { repository: repo };

    let result = handler.handle(DepositPayload {
        id: 100,
        amount: 10,
    });

    println!("{:?}", &repo2.get_events());

    match result {
        Ok(()) => println!("Money deposited"),
        _ => panic!("Depositing failed"),
    }
}

fn example_withdraw_money() {
    let initial_events = vec![
        BankAccountEvent::acc_opened(100, 20),
        BankAccountEvent::credited(100, 49),
    ];

    let repo = Arc::new(InMemoryBankAccountRepository::new());

    match repo.save_events(initial_events) {
        Ok(()) => println!("Setup OK"),
        _ => panic!("Setup failed"),
    }

    let repo2 = repo.clone();

    let handler = WithdrawHandler { repository: repo };

    let result = handler.handle(WithdrawPayload { id: 100, amount: 4 });

    println!("{:?}", &repo2.get_events());

    match result {
        Ok(()) => println!("Money withdrawn"),
        _ => panic!("Withdrawing failed"),
    }
}
