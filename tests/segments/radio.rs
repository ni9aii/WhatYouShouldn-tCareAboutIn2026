//! Tests for the radio segment.

use std::io;

use what_you_shouldnt_care_about_in_2026::input::{InputCommand, InputSource};
use what_you_shouldnt_care_about_in_2026::segments::radio::RadioSegment;
use what_you_shouldnt_care_about_in_2026::segments::Segment;
use what_you_shouldnt_care_about_in_2026::state::GameState;

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
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "no more input"));
        }
        let command = self.commands[self.index];
        self.index += 1;
        Ok(command)
    }
}

#[test]
fn radio_listen_captures_keyword() {
    let mut segment = RadioSegment;
    let mut state = GameState::default();
    let mut src = ScriptedInput::new(&[InputCommand::Character('y'), InputCommand::Confirm]);

    let outcome = segment.run(&mut state, &mut src).unwrap();

    assert_eq!(outcome, SegmentOutcome::Completed);
    assert!(state.profile.keywords.iter().any(|k| k == "broadcast"));
    assert!(state.completed_segments() >= 1);
}

#[test]
fn radio_skip_does_not_capture_keyword() {
    let mut segment = RadioSegment;
    let mut state = GameState::default();
    let mut src = ScriptedInput::new(&[InputCommand::Character('n'), InputCommand::Confirm]);

    let outcome = segment.run(&mut state, &mut src).unwrap();

    assert_eq!(outcome, SegmentOutcome::Completed);
    assert!(state.profile.keywords.is_empty());
}

#[test]
fn radio_cancel_preserves_progress() {
    let mut segment = RadioSegment;
    let mut state = GameState::default();
    state.complete_segment("elevator");
    let mut src = ScriptedInput::new(&[InputCommand::Cancel]);

    let outcome = segment.run(&mut state, &mut src).unwrap();

    assert_eq!(outcome, SegmentOutcome::Cancelled);
    assert_eq!(state.completed_segments(), 1);
}
