use crossterm::event::{KeyCode, KeyEvent};
use what_you_shouldnt_care_about_in_2026::input::{
    InputCommand, parse_floor_input, parse_yes_no_input,
};

#[test]
fn maps_terminal_keys_to_game_commands() {
    assert_eq!(
        InputCommand::from(KeyEvent::from(KeyCode::Enter)),
        InputCommand::Confirm
    );
    assert_eq!(
        InputCommand::from(KeyEvent::from(KeyCode::Esc)),
        InputCommand::Cancel
    );
    assert_eq!(
        InputCommand::from(KeyEvent::from(KeyCode::Char('q'))),
        InputCommand::Quit
    );
}

#[test]
fn parses_floor_input_and_clamps_to_valid_range() {
    assert_eq!(parse_floor_input("33"), Ok(Some(33)));
    assert_eq!(parse_floor_input("0"), Ok(Some(1)));
    assert_eq!(parse_floor_input("101"), Ok(Some(100)));
    assert_eq!(parse_floor_input("q"), Ok(None));
    assert!(parse_floor_input("thirteen").is_err());
}

#[test]
fn parses_yes_no_input_and_treats_enter_as_default_no() {
    assert_eq!(parse_yes_no_input("y"), Ok(Some(true)));
    assert_eq!(parse_yes_no_input("yes"), Ok(Some(true)));
    assert_eq!(parse_yes_no_input("n"), Ok(Some(false)));
    assert_eq!(parse_yes_no_input(""), Ok(Some(false)));
    assert_eq!(parse_yes_no_input("q"), Ok(None));
    assert!(parse_yes_no_input("maybe").is_err());
}
