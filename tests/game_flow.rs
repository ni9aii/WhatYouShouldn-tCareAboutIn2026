//! Scripted full-flow test: run all three segments, then verify the verdict
//! unlocks and produces non-empty output without a terminal.

use std::io;

use what_you_shouldnt_worry_about_in_2026::input::{InputCommand, InputSource};
use what_you_shouldnt_worry_about_in_2026::oracle;
use what_you_shouldnt_worry_about_in_2026::segments::{Segment, SegmentOutcome};
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

/// A scripted input that answers every yes/no with yes and confirms digits:
/// floor 50 -> panic yes -> engage yes (elevator), listen yes (radio),
/// look yes (mirror).
fn playthrough() -> Vec<InputCommand> {
    vec![
        InputCommand::Character('5'),
        InputCommand::Character('0'),
        InputCommand::Confirm,
        InputCommand::Character('y'),
        InputCommand::Confirm,
        InputCommand::Character('y'),
        InputCommand::Confirm,
        InputCommand::Character('y'),
        InputCommand::Confirm,
        InputCommand::Character('y'),
        InputCommand::Confirm,
        InputCommand::Character('y'),
        InputCommand::Confirm,
    ]
}

#[test]
fn all_segments_complete_and_unlock_verdict() {
    let mut state = GameState::default();
    assert!(!state.can_show_verdict());

    let mut segments = what_you_shouldnt_worry_about_in_2026::segments::all_segments();
    let mut input = ScriptedInput::new(&playthrough());

    for segment in segments.iter_mut() {
        let outcome = segment.run(&mut state, &mut input).unwrap();
        assert_eq!(outcome, SegmentOutcome::Completed);
    }

    assert_eq!(state.completed_segments(), 3);
    assert!(state.can_show_verdict());

    let verdict = oracle::generate(&state.profile);
    assert!(!verdict.is_empty());
    assert!(!verdict.contains("{}"));
}

#[test]
fn replaying_a_completed_segment_does_not_double_count() {
    let mut state = GameState::default();
    let mut segments = what_you_shouldnt_worry_about_in_2026::segments::all_segments();
    let mut input = ScriptedInput::new(&playthrough());

    for segment in segments.iter_mut() {
        let _ = segment.run(&mut state, &mut input).unwrap();
    }
    let first_count = state.completed_segments();

    // Running the elevator again must not consume input or award its profile
    // changes a second time.
    let mut elevator = what_you_shouldnt_worry_about_in_2026::segments::elevator::ElevatorSegment;
    let profile_before = state.profile.clone();
    let mut no_input = ScriptedInput::new(&[]);
    let outcome = elevator.run(&mut state, &mut no_input).unwrap();
    assert_eq!(outcome, SegmentOutcome::Completed);
    assert_eq!(state.completed_segments(), first_count);
    assert_eq!(state.profile, profile_before);
}

/// Two games started from the same seed and driven by the same inputs must
/// produce identical profiles and verdicts (reproducibility for replays/tests).
#[test]
fn same_seed_yields_identical_playthrough() {
    fn run() -> (GameState, String) {
        let mut state = GameState::with_seed(42);
        let mut segments = what_you_shouldnt_worry_about_in_2026::segments::all_segments();
        let mut input = ScriptedInput::new(&playthrough());
        for segment in segments.iter_mut() {
            let _ = segment.run(&mut state, &mut input).unwrap();
        }
        let verdict = oracle::generate(&state.profile);
        (state, verdict)
    }

    let (a, verdict_a) = run();
    let (b, verdict_b) = run();
    assert_eq!(a.profile, b.profile);
    assert_eq!(verdict_a, verdict_b);
}
