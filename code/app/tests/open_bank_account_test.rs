use app::mybank::prelude::*;

#[test]
fn opening_a_bank_account_emits_account_opened_event() {
    // Arrange
    let open_bank_account = BankAccountCommand::open_acc(100, 20);
    let expected = Ok(vec![BankAccountEvent::acc_opened(100, 20)]);

    // Act
    let result = BankAccountAggregate::handle(open_bank_account);

    // Assert
    assert_eq!(expected, result);
}

#[test]
fn account_opened_event_will_set_state_correctly() {
    // Arrange
    let events = vec![BankAccountEvent::acc_opened(100, 20)];
    let expected = Ok(BankAccountState {
        id: 100,
        customer_id: 20,
    });

    // Act
    let result = BankAccountAggregate::apply_events(events);

    // Assert
    assert_eq!(expected, result);
}
