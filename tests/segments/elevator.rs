//! Tests for the elevator segment.

use std::io;

use what_you_shouldnt_care_about_in_2026::input::{InputCommand, InputSource};
use what_you_shouldnt_care_about_in_2026::segments::elevator::ElevatorSegment;
use what_you_shouldnt_care_about_in_2026::segments::Segment;
use what_you_shouldnt_care_about_in_2026::state::{
    ElevatorEventCategory, GameState, PlayerAspects,
};

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

fn yes() -> InputCommand {
    InputCommand::Character('y')
}
fn no() -> InputCommand {
    InputCommand::Character('n')
}

#[test]
fn elevator_completes_and_records_progress() {
    let mut segment = ElevatorSegment;
    let mut state = GameState::default();
    // floor 90 -> Global event; panic -> yes; engage -> yes.
    let mut src = ScriptedInput::new(&[
        InputCommand::Character('9'),
        InputCommand::Character('0'),
        InputCommand::Confirm,
        yes(),
        InputCommand::Confirm,
        yes(),
        InputCommand::Confirm,
    ]);

    let outcome = segment.run(&mut state, &mut src).unwrap();

    assert_eq!(outcome, SegmentOutcome::Completed);
    assert!(state.completed_segments() >= 1);
    // floor>=70 -> +geopolitical (8) and +social (8); global event engage -> +geopolitical (6)
    assert!(state.profile.geopolitical_awareness >= 50 + 8 + 6);
}

#[test]
fn low_floor_selects_mystical_event() {
    assert_eq!(ElevatorEventCategory::from_floor(10), ElevatorEventCategory::Mystical);
    assert_eq!(ElevatorEventCategory::from_floor(50), ElevatorEventCategory::Social);
    assert_eq!(ElevatorEventCategory::from_floor(90), ElevatorEventCategory::Global);
}

#[test]
fn elevator_cancel_preserves_progress() {
    let mut segment = ElevatorSegment;
    let mut state = GameState::default();
    state.complete_segment("radio");
    // Quit right at the floor prompt.
    let mut src = ScriptedInput::new(&[InputCommand::Quit]);

    let outcome = segment.run(&mut state, &mut src).unwrap();

    assert_eq!(outcome, SegmentOutcome::Cancelled);
    assert_eq!(state.completed_segments(), 1);
}

#[test]
fn elevator_engage_no_event_does_not_boost_axis() {
    let mut state = GameState::default();
    let before = state.profile.baseline_anxiety;
    let feedback = state.apply_elevator_event(ElevatorEventCategory::Global, false);
    assert!(feedback.contains("ignore"));
    assert_eq!(state.profile.baseline_anxiety, before);
}

#[test]
fn default_profile_within_range() {
    let profile = PlayerAspects::default();
    assert!(profile.social_engagement <= 100);
}
