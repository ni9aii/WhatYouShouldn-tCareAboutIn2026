use std::io::{self, IsTerminal, Write, stdout};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use what_you_shouldnt_care_about_in_2026::{input, oracle, state::GameState};

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !stdout().is_terminal() {
        return run_non_interactive_demo();
    }

    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        execute!(stdout(), EnterAlternateScreen)?;
        terminal.draw(|frame| {
            let area = frame.area();
            let text = ratatui::widgets::Paragraph::new(
                "WHAT YOU SHOULDN'T CARE ABOUT IN 2026\n\nMVP-1 vertical slice\n\nPress Enter to start the elevator segment. Press q to quit.",
            )
            .block(ratatui::widgets::Block::bordered().title("Aura 2026"));
            frame.render_widget(text, area);
        })?;

        if !wait_for_start()? {
            break;
        }

        disable_raw_mode()?;
        execute!(
            stdout(),
            LeaveAlternateScreen,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;
        enable_raw_mode()?;
        let mut state = GameState::default();
        if !play_session(&mut state, &mut terminal)? {
            break;
        }

        if !wait_for_replay()? {
            break;
        }
    }

    disable_raw_mode()?;
    Ok(())
}

fn wait_for_start() -> io::Result<bool> {
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => return Ok(true),
                KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(false),
                _ => {}
            }
        }
    }
}

fn play_session(
    state: &mut GameState,
    _terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    print!(
        "ELEVATOR SEGMENT\r\n\r\nChoose a floor from 1 to 100.\r\nPress Enter after typing the floor, or q to quit.\r\n"
    );
    stdout().flush()?;

    let floor = match read_number("Elevator is waiting. Type a floor (1-100), or q to quit: ")? {
        Some(floor) => floor,
        None => return Ok(false),
    };
    let panic = match read_yes_no("\r\nThe elevator shudders between floors. Panic? [y/N]: ")? {
        Some(panic) => panic,
        None => return Ok(false),
    };
    let feedback = state.apply_elevator_decision(floor, panic);
    state.complete_segment("elevator");
    print!("\r\n{feedback}\r\n");

    let listen = match read_yes_no("\r\nThe radio starts broadcasting. Listen? [y/N]: ")? {
        Some(listen) => listen,
        None => return Ok(false),
    };
    let feedback = state.apply_radio_decision(listen);
    state.complete_segment("radio");
    print!("\r\n{feedback}\r\n");

    let look = match read_yes_no("\r\nA mirror appears in the corridor. Look into it? [y/N]: ")? {
        Some(look) => look,
        None => return Ok(false),
    };
    let feedback = state.apply_mirror_decision(look);
    state.complete_segment("mirror");
    print!("\r\n{feedback}\r\n");

    let verdict = if state.can_show_verdict() {
        oracle::generate(&state.profile)
    } else {
        format!(
            "{}\n\nComplete at least two more segments to unlock the Oracle verdict.",
            "ORACLE VERDICT LOCKED"
        )
    };
    print!("\r\n{}\r\n", normalize_terminal_text(&verdict));
    stdout().flush()?;
    Ok(true)
}

fn normalize_terminal_text(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\n', "\r\n")
}

fn wait_for_replay() -> io::Result<bool> {
    print!("\r\nPress r to play again, Enter or q to quit.\r\n");
    stdout().flush()?;
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('r') | KeyCode::Char('R') => return Ok(true),
                KeyCode::Enter | KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(false),
                _ => {}
            }
        }
    }
}

fn read_number(prompt: &str) -> io::Result<Option<u32>> {
    let mut input = String::new();
    print!("{prompt}");
    stdout().flush()?;
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') if input.is_empty() => return Ok(None),
                KeyCode::Char(character) if character.is_ascii_digit() => {
                    input.push(character);
                    print!("{character}");
                    stdout().flush()?;
                }
                KeyCode::Backspace => {
                    if input.pop().is_some() {
                        print!("\u{8} \u{8}");
                        stdout().flush()?;
                    }
                }
                KeyCode::Enter => {
                    if let Ok(Some(value)) = input::parse_floor_input(&input) {
                        print!("\r\n");
                        stdout().flush()?;
                        return Ok(Some(value));
                    }
                    if input::parse_floor_input(&input) == Ok(None) {
                        return Ok(None);
                    }
                    print!("\r\nPlease enter a number from 1 to 100, or q to quit.\r\n");
                    input.clear();
                    print!("{prompt}");
                    stdout().flush()?;
                }
                _ => {}
            }
        }
    }
}

fn read_yes_no(prompt: &str) -> io::Result<Option<bool>> {
    print!("{prompt}");
    stdout().flush()?;
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    print!("\r\ny\r\n");
                    stdout().flush()?;
                    return Ok(Some(true));
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Enter => {
                    print!("\r\nn\r\n");
                    stdout().flush()?;
                    return Ok(Some(false));
                }
                KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(None),
                _ => {}
            }
        }
    }
}

fn run_non_interactive_demo() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = GameState::default();
    let feedback = state.apply_elevator_decision(90, false);
    state.complete_segment("elevator");
    println!("{feedback}");

    let feedback = state.apply_radio_decision(false);
    state.complete_segment("radio");
    println!("\n{feedback}");

    let feedback = state.apply_mirror_decision(true);
    state.complete_segment("mirror");
    println!("\n{feedback}\n\n{}", oracle::generate(&state.profile));
    Ok(())
}
