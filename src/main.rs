use std::io::{self, IsTerminal, Write, stdout};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use what_you_shouldnt_care_about_in_2026::{
    input::{InputCommand, InputSource, TerminalGuard, TerminalInput},
    oracle,
    segments::{self, SegmentOutcome},
    state::GameState,
    ui,
};

/// Outcome of the replay prompt.
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

/// Run a single session driven by the segment menu, then show the verdict and
/// the replay prompt. Returns whether the player asked to replay.
fn play_session(state: &mut GameState, input: &mut dyn InputSource) -> io::Result<bool> {
    let mut segments = segments::all_segments();
    let entries = segments::segment_entries();

    loop {
        print!("{}", ui::format_menu(&entries, state.completed_set()));
        match read_menu_selection(input, segments.len())? {
            MenuSelection::Index(index) => {
                let outcome = segments[index].run(state, input)?;
                match outcome {
                    SegmentOutcome::Completed => {}
                    SegmentOutcome::Cancelled => {
                        print!("\r\nReturned to the menu.\r\n");
                    }
                }
            }
            MenuSelection::Verdict => {
                show_verdict(state)?;
                return handle_replay(input);
            }
            MenuSelection::Quit => return Ok(false),
        }

        if state.can_show_verdict() {
            print!(
                "\r\nThe Oracle verdict is available. Press v for the verdict, or pick another segment.\r\n"
            );
        }
    }
}

enum MenuSelection {
    Index(usize),
    Verdict,
    Quit,
}

/// Read a menu choice: number keys pick a segment, `v` requests the verdict
/// (only when available), `Esc`/`q` quit.
fn read_menu_selection(input: &mut dyn InputSource, count: usize) -> io::Result<MenuSelection> {
    loop {
        match input.read_command()? {
            InputCommand::Character(c @ '1'..='9') => {
                let index = c as usize - '1' as usize;
                if index < count {
                    return Ok(MenuSelection::Index(index));
                }
            }
            InputCommand::Character('v') | InputCommand::Character('V') => {
                return Ok(MenuSelection::Verdict);
            }
            InputCommand::Quit => return Ok(MenuSelection::Quit),
            InputCommand::Cancel => return Ok(MenuSelection::Quit),
            _ => {}
        }
    }
}

fn show_verdict(state: &GameState) -> io::Result<()> {
    let verdict = oracle::generate(&state.profile);
    print!("\r\n{}\r\n", verdict.replace('\n', "\r\n"));
    stdout().flush()?;
    Ok(())
}

fn handle_replay(input: &mut dyn InputSource) -> io::Result<bool> {
    match wait_for_replay(input)? {
        ReplayOutcome::Replay => Ok(true),
        ReplayOutcome::Quit => Ok(false),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !stdout().is_terminal() {
        return run_non_interactive_demo();
    }

    let mut input = TerminalInput;

    // RAII guard: raw mode is disabled and the alt screen left on any return
    // or panic, so the terminal is never left frozen.
    let _guard = TerminalGuard::enter()?;

    loop {
        execute!(stdout(), EnterAlternateScreen)?;
        {
            let backend = CrosstermBackend::new(stdout());
            let mut terminal = Terminal::new(backend)?;
            terminal.draw(|frame| {
                let area = frame.area();
                let text = ratatui::widgets::Paragraph::new(
                    "WHAT YOU SHOULDN'T CARE ABOUT IN 2026\n\nMVP-1 vertical slice\n\nPress Enter to enter the broadcast. Press Esc or q to quit.",
                )
                .block(ratatui::widgets::Block::bordered().title("Aura 2026"));
                frame.render_widget(text, area);
            })?;
        }

        // Wait for the player's choice WHILE the title is still on screen.
        let outcome = loop {
            match input.read_command()? {
                InputCommand::Confirm => break StartOutcome::Start,
                InputCommand::Quit | InputCommand::Cancel => break StartOutcome::Quit,
                _ => {}
            }
        };

        execute!(
            stdout(),
            LeaveAlternateScreen,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;

        match outcome {
            StartOutcome::Start => {}
            StartOutcome::Quit => break,
        }

        ui::render_onboarding();

        let mut state = GameState::default();
        if !play_session(&mut state, &mut input)? {
            break;
        }
    }

    Ok(())
}

enum StartOutcome {
    Start,
    Quit,
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
