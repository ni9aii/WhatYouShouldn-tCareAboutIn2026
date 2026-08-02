use std::io::{self, IsTerminal, Write, stdout};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use what_you_shouldnt_care_about_in_2026::{oracle, state::GameState};

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
    let mut state = GameState::default();

    terminal.draw(|frame| {
        let area = frame.area();
        let text = ratatui::widgets::Paragraph::new(
            "WHAT YOU SHOULDN'T CARE ABOUT IN 2026\n\nMVP-1 vertical slice\n\nPress Enter to run the elevator segment. Press q to quit.",
        )
        .block(ratatui::widgets::Block::bordered().title("Aura 2026"));
        frame.render_widget(text, area);
    })?;

    let mut input = String::new();
    print!("\nMVP-1 is ready. Type a floor (1-100), or q to quit: ");
    stdout().flush()?;
    disable_raw_mode()?;
    io::stdin().read_line(&mut input)?;
    enable_raw_mode()?;

    if input.trim() != "q" {
        let floor = input.trim().parse::<u32>().unwrap_or(50).clamp(1, 100);
        let feedback = state.apply_elevator_decision(floor, false);
        state.complete_segment("elevator");
        let verdict = if state.can_show_verdict() {
            oracle::generate(&state.profile)
        } else {
            format!(
                "{}\n\nComplete at least two more segments to unlock the Oracle verdict.",
                "ORACLE VERDICT LOCKED"
            )
        };
        println!("\n{feedback}\n\n{verdict}");
        println!("\nPress Enter to exit.");
        disable_raw_mode()?;
        let mut ignored = String::new();
        io::stdin().read_line(&mut ignored)?;
        enable_raw_mode()?;
    }

    Ok(())
}

fn run_non_interactive_demo() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = GameState::default();
    let feedback = state.apply_elevator_decision(90, false);
    state.complete_segment("elevator");
    println!(
        "{feedback}\n\nORACLE VERDICT LOCKED\n\nComplete at least two more segments to unlock the Oracle verdict."
    );
    Ok(())
}
