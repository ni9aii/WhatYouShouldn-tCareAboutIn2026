use crossterm::event::{KeyCode, KeyEvent};

pub fn parse_floor_input(input: &str) -> Result<Option<u32>, &'static str> {
    let trimmed = input.trim();
    if trimmed.eq_ignore_ascii_case("q") {
        return Ok(None);
    }

    trimmed
        .parse::<u32>()
        .map(|floor| Some(floor.clamp(1, 100)))
        .map_err(|_| "floor must be a number from 1 to 100")
}

pub fn parse_yes_no_input(input: &str) -> Result<Option<bool>, &'static str> {
    match input.trim().to_ascii_lowercase().as_str() {
        "y" | "yes" => Ok(Some(true)),
        "" | "n" | "no" => Ok(Some(false)),
        "q" => Ok(None),
        _ => Err("answer must be y, n, or q"),
    }
}

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
