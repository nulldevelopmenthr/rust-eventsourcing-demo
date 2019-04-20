mod bank;

use crate::bank::account::prelude::*;
use eventsourcing::Aggregate;

fn main() {
    open_bank_account_example();
    deposit_example();
    println!("Done!");
}

fn open_bank_account_example() {
    // Arrange
    let mut agg = BankAccountAggregate::default();
    let cmd = OpenBankAccount::new(123, 5000);
    let expected_agg_result = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));

    // Act
    let events = agg.execute(cmd).unwrap();

    for event in events {
        agg.apply(event).unwrap();
    }

    // Assert
    assert_eq!(expected_agg_result, agg);
}

fn deposit_example() {
    // Arrange
    let mut agg = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
    let cmd = DepositMoney::new(123, 49);
    let expected_balance = 49;

    // Act
    let events = agg.execute(cmd).unwrap();

    for event in events {
        agg.apply(event).unwrap();
    }

    // Assert

    if let BankAccountAggregate::Opened(state) = agg {
        assert_eq!(expected_balance, state.balance);
    } else {
        panic!("Aggregate not in Opened state");
    }
}
