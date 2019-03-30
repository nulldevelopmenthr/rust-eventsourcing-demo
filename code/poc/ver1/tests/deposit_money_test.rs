use ver1::prelude::*;

#[test]
fn depositing_money_emits_credited_event() {
    // Arrange
    let initial_events = vec![BankAccountEvent::acc_opened(100, 20)];
    let deposit = BankAccountCommand::deposit(100, 49);
    let expected = Ok(vec![BankAccountEvent::credited(100, 49)]);

    // Act
    let initial_state = BankAccountAggregate::apply_events(initial_events).unwrap();
    let result = BankAccountAggregate::handle(Some(initial_state), deposit);

    // Assert
    assert_eq!(expected, result);
}

#[test]
fn first_account_credited_event_will_set_state_correctly() {
    // Arrange
    let events = vec![
        BankAccountEvent::acc_opened(100, 20),
        BankAccountEvent::credited(100, 49),
    ];
    let expected = Ok(BankAccountState {
        id: 100,
        customer_id: 20,
        balance: 49,
    });

    // Act
    let result = BankAccountAggregate::apply_events(events);

    // Assert
    assert_eq!(expected, result);
}

#[test]
fn account_credited_event_will_set_state_correctly() {
    // Arrange
    let events = vec![
        BankAccountEvent::acc_opened(100, 20),
        BankAccountEvent::credited(100, 49),
        BankAccountEvent::credited(100, 49),
    ];
    let expected = Ok(BankAccountState {
        id: 100,
        customer_id: 20,
        balance: 98,
    });

    // Act
    let result = BankAccountAggregate::apply_events(events);

    // Assert
    assert_eq!(expected, result);
}
