use std::io;

use crossterm::event::{KeyEvent, KeyEventKind};
use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};

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

/// Outcome of an interactive prompt driven by an `InputSource`.
///
/// `Value` carries the parsed result; `Cancel` means the player pressed Esc and
/// wants to return to the menu without losing progress; `Quit` means the player
/// asked to leave the game entirely (q / EOF).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadOutcome<T> {
    Value(T),
    Cancel,
    Quit,
}

/// Read a numeric choice (floor, frequency, etc.) through an abstract source.
///
/// Accepts ASCII digits, `Enter` confirms the buffered number, `Backspace`
/// erases the last digit, `Esc` cancels, and `q`/`Q` quits. Empty or invalid
/// input is rejected with a retry prompt rather than accepted.
pub fn read_number(source: &mut dyn InputSource, prompt: &str) -> io::Result<ReadOutcome<u32>> {
    let mut buffer = String::new();
    print!("{prompt}");
    use std::io::Write;
    std::io::stdout().flush()?;
    loop {
        match source.read_command()? {
            InputCommand::Character(character) if character.is_ascii_digit() => {
                buffer.push(character);
                print!("{character}");
                std::io::stdout().flush()?;
            }
            InputCommand::Backspace => {
                if buffer.pop().is_some() {
                    print!("\u{8} \u{8}");
                    std::io::stdout().flush()?;
                }
            }
            InputCommand::Confirm => match parse_floor_input(&buffer) {
                Ok(Some(value)) => {
                    print!("\r\n");
                    std::io::stdout().flush()?;
                    break Ok(ReadOutcome::Value(value));
                }
                Ok(None) => break Ok(ReadOutcome::Quit),
                Err(_) => {
                    print!("\r\nPlease enter a number from 1 to 100.\r\n");
                    buffer.clear();
                    print!("{prompt}");
                    std::io::stdout().flush()?;
                }
            },
            InputCommand::Cancel => break Ok(ReadOutcome::Cancel),
            InputCommand::Quit => break Ok(ReadOutcome::Quit),
            _ => {}
        }
    }
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
    Selected(bool),
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
                break Ok(Choice::Selected(pending.unwrap_or(false)));
            }
            InputCommand::Cancel => break Ok(Choice::Cancelled),
            InputCommand::Quit => break Ok(Choice::Quit),
            _ => {}
        }
    }
}

/// RAII guard that restores the terminal to a usable state on drop.
///
/// `TerminalInput` switches the terminal into raw mode for menu navigation.
/// If the game returns early or panics, the guard's `Drop` impl guarantees raw
/// mode is disabled and the alternate screen is left, so the user is never
/// left with a frozen, unresponsive terminal.
pub struct TerminalGuard;

impl TerminalGuard {
    /// Enter raw mode; the terminal is restored when the guard is dropped.
    pub fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        Ok(TerminalGuard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), LeaveAlternateScreen);
    }
}
