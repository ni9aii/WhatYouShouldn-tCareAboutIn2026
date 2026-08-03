//! Tests for the mirror segment.

use std::io;

use what_you_shouldnt_worry_about_in_2026::input::{InputCommand, InputSource};
use what_you_shouldnt_worry_about_in_2026::segments::mirror::MirrorSegment;
use what_you_shouldnt_worry_about_in_2026::segments::Segment;
use what_you_shouldnt_worry_about_in_2026::state::GameState;

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
fn mirror_look_completes() {
    let mut segment = MirrorSegment;
    let mut state = GameState::default();
    let mut src = ScriptedInput::new(&[InputCommand::Character('y'), InputCommand::Confirm]);

    let outcome = segment.run(&mut state, &mut src).unwrap();

    assert_eq!(outcome, SegmentOutcome::Completed);
    assert!(state.completed_segments() >= 1);
}

#[test]
fn mirror_cancel_preserves_progress() {
    let mut segment = MirrorSegment;
    let mut state = GameState::default();
    state.complete_segment("elevator");
    state.complete_segment("radio");
    let mut src = ScriptedInput::new(&[InputCommand::Cancel]);

    let outcome = segment.run(&mut state, &mut src).unwrap();

    assert_eq!(outcome, SegmentOutcome::Cancelled);
    assert_eq!(state.completed_segments(), 2);
}
