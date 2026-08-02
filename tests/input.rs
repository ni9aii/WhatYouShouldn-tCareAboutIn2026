use crossterm::event::{KeyCode, KeyEvent};
use what_you_shouldnt_care_about_in_2026::input::InputCommand;

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
