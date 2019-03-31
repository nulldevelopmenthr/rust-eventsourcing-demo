use crate::command::DepositMoney;
use crate::command::OpenBankAccount;
use crate::command::WithdrawMoney;
use crate::deposit::DepositHandler;
use crate::event::BankAccountEvent;
use crate::event_store::BankAccountEventStore;
use crate::event_store::InMemoryBankAccountEventStore;
use crate::open_bank_account::OpenBankAccountHandler;
use crate::repository::BankAccountRepository;
use crate::withdraw::WithdrawHandler;
use std::sync::Arc;

pub fn examples() {
    example_open_bank_account();
    example_deposit_money();
    example_withdraw_money();
    example_withdraw_refused();

    println!("Done!");
}

fn example_open_bank_account() {
    let (repo, event_store) = build_repo(Vec::new());
    let handler = OpenBankAccountHandler::new(repo);

    let result = handler.handle(OpenBankAccount::new(100, 20));

    println!("{:?}", &event_store.get_events(1));

    match result {
        Ok(()) => println!("Bank account opened"),
        _ => panic!("Opening bank account failed"),
    }
}

fn example_deposit_money() {
    let initial_events = vec![BankAccountEvent::acc_opened(100, 20)];

    let (repo, event_store) = build_repo(initial_events);

    let handler = DepositHandler::new(repo);

    let result = handler.handle(DepositMoney::new(100, 10));

    println!("{:?}", &event_store.get_events(1));

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
    let (repo, event_store) = build_repo(initial_events);

    let handler = WithdrawHandler::new(repo);

    let result = handler.handle(WithdrawMoney::new(100, 40));

    println!("{:?}", &event_store.get_events(1));

    match result {
        Ok(()) => println!("Money withdrawn"),
        _ => panic!("Withdrawing failed"),
    }
}

fn example_withdraw_refused() {
    let initial_events = vec![
        BankAccountEvent::acc_opened(100, 20),
        BankAccountEvent::credited(100, 49),
    ];
    let (repo, event_store) = build_repo(initial_events);

    let handler = WithdrawHandler::new(repo);

    let result = handler.handle(WithdrawMoney::new(100, 50));

    println!("{:?}", &event_store.get_events(1));

    match result {
        Ok(()) => println!("Money withdrawal refused"),
        _ => panic!("Withdrawal refusing failed"),
    }
}

type BuildRepoResult = (
    Arc<BankAccountRepository>,
    Arc<InMemoryBankAccountEventStore>,
);

fn build_repo(initial_events: Vec<BankAccountEvent>) -> BuildRepoResult {
    let event_store = Arc::new(InMemoryBankAccountEventStore::new());
    let event_store2 = event_store.clone();
    let repo = Arc::new(BankAccountRepository {
        event_store: event_store,
    });

    match event_store2.save_events(initial_events) {
        Ok(()) => println!("Initial events added"),
        _ => panic!("Setting up initial events failed"),
    }

    (repo, event_store2)
}
