use std::io::{self, IsTerminal, Write, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
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
            "WHAT YOU SHOULDN'T CARE ABOUT IN 2026\n\nMVP-1 vertical slice\n\nPress Enter to start the elevator segment. Press q to quit.",
        )
        .block(ratatui::widgets::Block::bordered().title("Aura 2026"));
        frame.render_widget(text, area);
    })?;

    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => break,
                KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                _ => {}
            }
        }
    }

    terminal.draw(|frame| {
        let area = frame.area();
        let text = ratatui::widgets::Paragraph::new(
            "ELEVATOR SEGMENT\n\nChoose a floor from 1 to 100.\nPress Enter after typing the floor, or q to quit.",
        )
        .block(ratatui::widgets::Block::bordered().title("Aura 2026"));
        frame.render_widget(text, area);
    })?;

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    let floor = read_number("Elevator is waiting. Type a floor (1-100), or q to quit: ")?;
    let feedback = state.apply_elevator_decision(floor, false);
    state.complete_segment("elevator");
    println!("\n{feedback}");

    let listen = read_yes_no("\nThe radio starts broadcasting. Listen? [y/N]: ")?;
    let feedback = state.apply_radio_decision(listen);
    state.complete_segment("radio");
    println!("\n{feedback}");

    let look = read_yes_no("\nA mirror appears in the corridor. Look into it? [y/N]: ")?;
    let feedback = state.apply_mirror_decision(look);
    state.complete_segment("mirror");
    println!("\n{feedback}");

    let verdict = if state.can_show_verdict() {
        oracle::generate(&state.profile)
    } else {
        format!(
            "{}\n\nComplete at least two more segments to unlock the Oracle verdict.",
            "ORACLE VERDICT LOCKED"
        )
    };
    println!("\n{verdict}");
    println!("\nPress Enter to exit.");
    let mut ignored = String::new();
    io::stdin().read_line(&mut ignored)?;

    Ok(())
}

fn read_number(prompt: &str) -> io::Result<u32> {
    let mut input = String::new();
    loop {
        print!("{prompt}");
        stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("q") {
            std::process::exit(0);
        }
        if let Ok(value) = trimmed.parse::<u32>() {
            return Ok(value.clamp(1, 100));
        }
        println!("Please enter a number from 1 to 100, or q to quit.");
    }
}

fn read_yes_no(prompt: &str) -> io::Result<bool> {
    let mut input = String::new();
    loop {
        print!("{prompt}");
        stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "" | "n" | "no" => return Ok(false),
            "q" => std::process::exit(0),
            _ => println!("Please answer y, n, or q to quit."),
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
