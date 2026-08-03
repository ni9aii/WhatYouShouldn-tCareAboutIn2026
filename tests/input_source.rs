//! Tests for the abstract `InputSource` boundary and command mapping.
//!
//! These run without a terminal. The `ScriptedInput` source drives the same
//! game logic that the real `TerminalInput` feeds in interactive mode, so the
//! Esc/q/Confirm handling exercised here is exactly what the player gets.

use std::io;

use what_you_shouldnt_care_about_in_2026::input::{InputCommand, InputSource};

/// A deterministic source of commands used to drive game logic in tests.
struct ScriptedInput {
    commands: Vec<InputCommand>,
    index: usize,
}

impl ScriptedInput {
    fn new(commands: &[InputCommand]) -> Self {
        Self {
            commands: commands.to_vec(),
            index: 0,
        }
    }
}

impl InputSource for ScriptedInput {
    fn read_command(&mut self) -> io::Result<InputCommand> {
        if self.index >= self.commands.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "no more input",
            ));
        }
        let command = self.commands[self.index];
        self.index += 1;
        Ok(command)
    }
}

#[test]
fn scripted_source_yields_commands_in_order() {
    let mut source = ScriptedInput::new(&[
        InputCommand::Character('5'),
        InputCommand::Confirm,
        InputCommand::Cancel,
    ]);

    assert_eq!(source.read_command().unwrap(), InputCommand::Character('5'));
    assert_eq!(source.read_command().unwrap(), InputCommand::Confirm);
    assert_eq!(source.read_command().unwrap(), InputCommand::Cancel);
}

#[test]
fn esc_maps_to_cancel_not_unknown() {
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    let mut event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    event.kind = KeyEventKind::Press;
    assert_eq!(InputCommand::from(event), InputCommand::Cancel);
}

#[test]
fn key_release_events_are_ignored_as_unknown() {
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    let mut release = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
    release.kind = KeyEventKind::Release;
    assert_eq!(InputCommand::from(release), InputCommand::Unknown);
}
