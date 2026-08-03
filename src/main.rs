use std::io::{self, IsTerminal, Write, stdout};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use what_you_shouldnt_care_about_in_2026::{
    input::{self, Choice, InputCommand, InputSource, TerminalInput},
    oracle,
    state::GameState,
};

/// Outcome of an interactive prompt.
///
/// `Value` carries the parsed result. `Cancel` means the player pressed Esc and
/// wants to return to the menu without losing progress. `Quit` means the player
/// asked to leave the game entirely (q / EOF).
enum ReadOutcome<T> {
    Value(T),
    Cancel,
    Quit,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !stdout().is_terminal() {
        return run_non_interactive_demo();
    }

    let mut input = TerminalInput;

    loop {
        // Enter the alternate screen only for the title screen, which is drawn
        // through ratatui. The gameplay section below writes plain text, so we
        // leave the alternate screen before it to avoid mixing ratatui's buffer
        // management with raw `print!` output (which previously crashed on replay).
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        {
            let backend = CrosstermBackend::new(stdout());
            let mut terminal = Terminal::new(backend)?;
            terminal.draw(|frame| {
                let area = frame.area();
                let text = ratatui::widgets::Paragraph::new(
                    "WHAT YOU SHOULDN'T CARE ABOUT IN 2026\n\nMVP-1 vertical slice\n\nPress Enter to start the elevator segment. Press Esc or q to quit.",
                )
                .block(ratatui::widgets::Block::bordered().title("Aura 2026"));
                frame.render_widget(text, area);
            })?;
        }
        execute!(
            stdout(),
            LeaveAlternateScreen,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;

        match wait_for_start(&mut input)? {
            StartOutcome::Start => {}
            StartOutcome::Quit => break,
        }

        let mut state = GameState::default();
        match play_session(&mut state, &mut input)? {
            SessionOutcome::Replay => {}
            SessionOutcome::Quit => break,
        }

        match wait_for_replay(&mut input)? {
            ReplayOutcome::Replay => {}
            ReplayOutcome::Quit => break,
        }

        disable_raw_mode()?;
    }

    disable_raw_mode()?;
    Ok(())
}

enum StartOutcome {
    Start,
    Quit,
}

fn wait_for_start(input: &mut dyn InputSource) -> io::Result<StartOutcome> {
    let outcome = loop {
        match input.read_command()? {
            InputCommand::Confirm => break StartOutcome::Start,
            InputCommand::Quit => break StartOutcome::Quit,
            // Esc quits from the start screen: there is no progress to preserve
            // yet, and the spec requires Esc to drop back to a safe exit.
            InputCommand::Cancel => break StartOutcome::Quit,
            _ => {}
        }
    };
    Ok(outcome)
}

enum SessionOutcome {
    Replay,
    Quit,
}

fn play_session(state: &mut GameState, input: &mut dyn InputSource) -> io::Result<SessionOutcome> {
    print!(
        "ELEVATOR SEGMENT\r\n\r\nChoose a floor from 1 to 100.\r\nType digits, then press Enter to confirm, Esc to return to the menu, or q to quit.\r\n"
    );
    stdout().flush()?;

    let floor = match read_number(input, "Elevator is waiting. Type a floor (1-100): ")? {
        ReadOutcome::Value(floor) => floor,
        ReadOutcome::Cancel => return Ok(SessionOutcome::Replay),
        ReadOutcome::Quit => return Ok(SessionOutcome::Quit),
    };
    let panic = match read_yes_no(
        input,
        "\r\nThe elevator shudders between floors. Panic? (y/n, then Enter): ",
    )? {
        ReadOutcome::Value(panic) => panic,
        ReadOutcome::Cancel => return Ok(SessionOutcome::Replay),
        ReadOutcome::Quit => return Ok(SessionOutcome::Quit),
    };
    let feedback = state.apply_elevator_decision(floor, panic);
    state.complete_segment("elevator");
    print!("\r\n{feedback}\r\n");

    let listen = match read_yes_no(
        input,
        "\r\nThe radio starts broadcasting. Listen? (y/n, then Enter): ",
    )? {
        ReadOutcome::Value(listen) => listen,
        ReadOutcome::Cancel => return Ok(SessionOutcome::Replay),
        ReadOutcome::Quit => return Ok(SessionOutcome::Quit),
    };
    let feedback = state.apply_radio_decision(listen);
    state.complete_segment("radio");
    print!("\r\n{feedback}\r\n");

    let look = match read_yes_no(
        input,
        "\r\nA mirror appears in the corridor. Look into it? (y/n, then Enter): ",
    )? {
        ReadOutcome::Value(look) => look,
        ReadOutcome::Cancel => return Ok(SessionOutcome::Replay),
        ReadOutcome::Quit => return Ok(SessionOutcome::Quit),
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
    Ok(SessionOutcome::Replay)
}

fn normalize_terminal_text(text: &str) -> String {
    text.replace('\r', "").replace('\n', "\r\n")
}

enum ReplayOutcome {
    Replay,
    Quit,
}

fn wait_for_replay(input: &mut dyn InputSource) -> io::Result<ReplayOutcome> {
    print!("\r\nPress r to play again, Enter or q to quit.\r\n");
    stdout().flush()?;
    let outcome = loop {
        match input.read_command()? {
            InputCommand::Character('r') | InputCommand::Character('R') => {
                break ReplayOutcome::Replay;
            }
            InputCommand::Confirm | InputCommand::Quit => break ReplayOutcome::Quit,
            InputCommand::Cancel => break ReplayOutcome::Quit,
            _ => {}
        }
    };
    Ok(outcome)
}

fn read_number(input: &mut dyn InputSource, prompt: &str) -> io::Result<ReadOutcome<u32>> {
    let mut buffer = String::new();
    print!("{prompt}");
    stdout().flush()?;
    loop {
        match input.read_command()? {
            InputCommand::Character(character) if character.is_ascii_digit() => {
                buffer.push(character);
                print!("{character}");
                stdout().flush()?;
            }
            InputCommand::Backspace => {
                if buffer.pop().is_some() {
                    print!("\u{8} \u{8}");
                    stdout().flush()?;
                }
            }
            InputCommand::Confirm => match input::parse_floor_input(&buffer) {
                Ok(Some(value)) => {
                    print!("\r\n");
                    stdout().flush()?;
                    break Ok(ReadOutcome::Value(value));
                }
                Ok(None) => break Ok(ReadOutcome::Quit),
                Err(_) => {
                    print!("\r\nPlease enter a number from 1 to 100.\r\n");
                    buffer.clear();
                    print!("{prompt}");
                    stdout().flush()?;
                }
            },
            InputCommand::Cancel => break Ok(ReadOutcome::Cancel),
            InputCommand::Quit => break Ok(ReadOutcome::Quit),
            _ => {}
        }
    }
}

fn read_yes_no(input: &mut dyn InputSource, prompt: &str) -> io::Result<ReadOutcome<bool>> {
    print!("{prompt}");
    stdout().flush()?;
    match input::prompt_choice(input)? {
        Choice::Confirmed => {
            print!("\r\ny\r\n");
            stdout().flush()?;
            Ok(ReadOutcome::Value(true))
        }
        Choice::Cancelled => {
            print!("\r\nn\r\n");
            stdout().flush()?;
            Ok(ReadOutcome::Value(false))
        }
        Choice::Quit => Ok(ReadOutcome::Quit),
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
