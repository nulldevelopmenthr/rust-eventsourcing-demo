mod command;
mod event;
mod model;
pub mod prelude;

use command::BankAccountCommand;
use model::BankAccountAggregate;

pub fn main() {
    example_open_bank_account();

    println!("Done!");
}

fn example_open_bank_account() {
    let open_bank_account = BankAccountCommand::open_acc(100, 20);

    let state = None;
    let events_result = BankAccountAggregate::handle(state, open_bank_account);

    let events = match events_result {
        Ok(events) => events,
        _ => Vec::new(),
    };

    println!("Events: {:?}", &events);

    let state_result = BankAccountAggregate::apply_events(events);

    let state = match state_result {
        Ok(state) => state,
        _ => panic!("NO STATE"),
    };

    println!("State: {:?}", state);
}
