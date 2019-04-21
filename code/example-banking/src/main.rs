mod bank;

use crate::bank::account::prelude::*;
use eventsourcing::eventstore::DummyEventStore;
use eventsourcing::Aggregate;

fn main() {
    open_bank_account_example1();
    open_bank_account_example2();
    deposit_example();
    withdraw_example();
    not_enough_funds_example();
    close_example();
    println!("Done!");
}

const ACCOUNT_ID: BankAccountId = 123;
const CUSTOMER_ID: CustomerId = 123;

fn open_bank_account_example1() {
    // Arrange
    let cmd = OpenBankAccount::new(ACCOUNT_ID, CUSTOMER_ID);
    let _event_store = DummyEventStore {};
    let repository = BankAccountRepository {};
    let handler = OpenBankAccountHandler::new(repository);

    // Act
    let result = handler.handle(cmd);

    // Arrange
    assert_eq!(Ok(()), result);
}

fn open_bank_account_example2() {
    // Arrange
    let mut agg = BankAccountAggregate::default();
    let cmd = OpenBankAccount::new(123, 5000);

    // Act
    let events = agg.execute(cmd).unwrap();

    for event in events {
        agg.apply(event).unwrap();
    }

    // Assert
    if let BankAccountAggregate::Opened(state, _) = agg {
        assert_eq!(123, state.id);
        assert_eq!(5000, state.customer_id);
        assert_eq!(0, state.balance);
    } else {
        panic!("Aggregate not in Opened state");
    }
}

fn deposit_example() {
    // Arrange
    let mut agg = BankAccountAggregate::default();
    agg.apply(BankAccountEvent::opened(123, 5000)).unwrap();
    let cmd = DepositMoney::new(123, 49);
    let expected_balance = 49;

    // Act
    let events = agg.execute(cmd).unwrap();

    for event in events {
        agg.apply(event).unwrap();
    }

    // Assert
    if let BankAccountAggregate::Opened(state, _) = agg {
        assert_eq!(expected_balance, state.balance);
    } else {
        panic!("Aggregate not in Opened state");
    }
}

fn withdraw_example() {
    // Arrange
    let mut agg = BankAccountAggregate::default();
    agg.apply(BankAccountEvent::opened(123, 5000)).unwrap();
    agg.apply(BankAccountEvent::credited(123, 50)).unwrap();
    let cmd = WithdrawMoney::new(123, 49);
    let expected_balance = 1;

    // Act
    let events = agg.execute(cmd).unwrap();

    for event in events {
        agg.apply(event).unwrap();
    }

    // Assert
    if let BankAccountAggregate::Opened(state, _) = agg {
        assert_eq!(expected_balance, state.balance);
    } else {
        panic!("Aggregate not in Opened state");
    }
}

fn not_enough_funds_example() {
    // Arrange
    let mut agg = BankAccountAggregate::default();
    agg.apply(BankAccountEvent::opened(123, 5000)).unwrap();
    let cmd = WithdrawMoney::new(123, 49);
    let expected_balance = 0;

    // Act
    let events = agg.execute(cmd).unwrap();

    for event in events {
        agg.apply(event).unwrap();
    }

    // Assert

    if let BankAccountAggregate::Opened(state, _) = agg {
        assert_eq!(expected_balance, state.balance);
    } else {
        panic!("Aggregate not in Opened state");
    }
}

fn close_example() {
    // Arrange
    let mut agg = BankAccountAggregate::default();
    agg.apply(BankAccountEvent::opened(123, 5000)).unwrap();
    let cmd = CloseBankAccount::new(123);
    let expected_balance = 0;

    // Act
    let events = agg.execute(cmd).unwrap();

    for event in events {
        agg.apply(event).unwrap();
    }

    // Assert
    if let BankAccountAggregate::Closed(state, _) = agg {
        assert_eq!(expected_balance, state.balance);
    } else {
        panic!("Aggregate not in Closed state");
    }
}
