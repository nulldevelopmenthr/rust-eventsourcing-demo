mod bank;

use crate::bank::account::prelude::*;
use eventsourcing::Aggregate;

fn main() {
    open_bank_account_example();
    println!("Done!");
}

fn open_bank_account_example() {
    // Arrange
    let mut agg = BankAccountAggregate::default();
    let cmd = OpenBankAccount::new(123, 5000);

    // Act
    let events = agg.execute(cmd).unwrap();

    for event in events {
        agg.apply(event).unwrap();
    }

    // Assert
    let expected_agg_result = BankAccountAggregate::Opened(BankAccountState::new(123, 5000));
    assert_eq!(expected_agg_result, agg);
}
