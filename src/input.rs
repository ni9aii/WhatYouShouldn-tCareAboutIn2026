use std::io;

use crossterm::event::{KeyEvent, KeyEventKind};

/// Convert a keyboard event into a game command.
///
/// Game logic must depend on `InputCommand`, never on raw `crossterm::Event`,
/// so the same logic can be driven by a scripted source in tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputCommand {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Cancel,
    Backspace,
    Quit,
    Character(char),
    Unknown,
}

impl From<KeyEvent> for InputCommand {
    fn from(event: KeyEvent) -> Self {
        // Ignore key-release events so a held key does not double-fire.
        if event.kind == KeyEventKind::Release {
            return Self::Unknown;
        }
        match event.code {
            crossterm::event::KeyCode::Up => Self::Up,
            crossterm::event::KeyCode::Down => Self::Down,
            crossterm::event::KeyCode::Left => Self::Left,
            crossterm::event::KeyCode::Right => Self::Right,
            crossterm::event::KeyCode::Enter => Self::Confirm,
            crossterm::event::KeyCode::Esc => Self::Cancel,
            crossterm::event::KeyCode::Backspace => Self::Backspace,
            crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Char('Q') => {
                Self::Quit
            }
            crossterm::event::KeyCode::Char(character) => Self::Character(character),
            _ => Self::Unknown,
        }
    }
}

/// A source of game commands, decoupled from the terminal.
///
/// The terminal adapter reads real key events; tests provide a scripted source.
pub trait InputSource {
    fn read_command(&mut self) -> io::Result<InputCommand>;
}

/// Reads real keyboard events from the terminal through `crossterm`.
pub struct TerminalInput;

impl InputSource for TerminalInput {
    fn read_command(&mut self) -> io::Result<InputCommand> {
        loop {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                let command = InputCommand::from(key);
                if command != InputCommand::Unknown {
                    break Ok(command);
                }
            }
        }
    }
}

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

/// Outcome of a confirm/cancel prompt driven by an `InputSource`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Confirmed,
    Cancelled,
    Quit,
}

/// Run a two-step yes/no selection through an abstract source.
///
/// The player first chooses `y`/`n` (which sets the pending answer), then
/// confirms with `Enter`. This avoids the previous behaviour where any key
/// was accepted instantly and the choice was never explicitly confirmed.
///
/// - `y`/`Y` selects yes; `n`/`N` selects no. Re-selecting changes the pending
///   answer. `Enter` confirms the pending answer (defaulting to no when none
///   was chosen). `Esc` cancels back to the menu; `q`/`Q` quits the game.
pub fn prompt_choice(source: &mut dyn InputSource) -> io::Result<Choice> {
    let mut pending: Option<bool> = None;
    loop {
        match source.read_command()? {
            InputCommand::Character('y') | InputCommand::Character('Y') => pending = Some(true),
            InputCommand::Character('n') | InputCommand::Character('N') => pending = Some(false),
            InputCommand::Confirm => {
                break Ok(if pending.unwrap_or(false) {
                    Choice::Confirmed
                } else {
                    Choice::Cancelled
                });
            }
            InputCommand::Cancel => break Ok(Choice::Cancelled),
            InputCommand::Quit => break Ok(Choice::Quit),
            _ => {}
        }
    }
}
