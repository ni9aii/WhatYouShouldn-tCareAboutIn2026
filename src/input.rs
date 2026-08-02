use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputCommand {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Cancel,
    Quit,
    Character(char),
    Unknown,
}

impl From<KeyEvent> for InputCommand {
    fn from(event: KeyEvent) -> Self {
        match event.code {
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            KeyCode::Enter => Self::Confirm,
            KeyCode::Esc => Self::Cancel,
            KeyCode::Char('q') | KeyCode::Char('Q') => Self::Quit,
            KeyCode::Char(character) => Self::Character(character),
            _ => Self::Unknown,
        }
    }
}
