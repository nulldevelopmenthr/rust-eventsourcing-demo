use app::mybank::prelude::*;

#[test]
fn withdrawing_too_much_money_emits_withdrawal_refused_event() {
    // Arrange
    let initial_events = vec![
        BankAccountEvent::acc_opened(100, 20),
        BankAccountEvent::credited(100, 49),
    ];
    let withdraw = BankAccountCommand::withdraw(100, 90);
    let expected = Ok(vec![BankAccountEvent::withdrawal_refused(100, 90, 49)]);

    // Act
    let state = BankAccountAggregate::apply_events(initial_events).unwrap();
    let result = BankAccountAggregate::handle(Some(state), withdraw);

    // Assert
    assert_eq!(expected, result);
}
