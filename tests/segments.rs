//! Tests for the `Segment` trait and the segment helpers.
//!
//! These run without a terminal: a scripted `InputSource` drives the same
//! logic a real `TerminalInput` feeds in interactive mode.

use std::io;

use what_you_shouldnt_worry_about_in_2026::input::{InputCommand, InputSource, ReadOutcome};
use what_you_shouldnt_worry_about_in_2026::segments::{Segment, SegmentOutcome};
use what_you_shouldnt_worry_about_in_2026::state::GameState;

/// A deterministic source of commands used to drive segment logic in tests.
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

/// A fake segment that records whether it ran and can be told to cancel.
struct FakeSegment {
    id: &'static str,
    title: &'static str,
    cancel: bool,
    ran: bool,
}

impl FakeSegment {
    fn new(id: &'static str, title: &'static str, cancel: bool) -> Self {
        Self {
            id,
            title,
            cancel,
            ran: false,
        }
    }
}

impl Segment for FakeSegment {
    fn id(&self) -> &'static str {
        self.id
    }

    fn title(&self) -> &'static str {
        self.title
    }

    fn run(
        &mut self,
        state: &mut GameState,
        _input: &mut dyn InputSource,
    ) -> io::Result<SegmentOutcome> {
        self.ran = true;
        if self.cancel {
            return Ok(SegmentOutcome::Cancelled);
        }
        state.complete_segment(self.id);
        Ok(SegmentOutcome::Completed)
    }
}

#[test]
fn fake_segment_marks_ran_and_completes() {
    let mut segment = FakeSegment::new("fake", "Fake", false);
    let mut state = GameState::default();
    let mut source = ScriptedInput::new(&[]);

    let outcome = segment.run(&mut state, &mut source).unwrap();

    assert!(segment.ran);
    assert_eq!(outcome, SegmentOutcome::Completed);
    assert_eq!(state.completed_segments(), 1);
}

#[test]
fn cancelling_segment_preserves_progress() {
    let mut segment = FakeSegment::new("fake", "Fake", true);
    let mut state = GameState::default();
    state.complete_segment("other");
    let mut source = ScriptedInput::new(&[]);

    let outcome = segment.run(&mut state, &mut source).unwrap();

    // The segment reports cancel but the previously completed progress remains.
    assert_eq!(outcome, SegmentOutcome::Cancelled);
    assert_eq!(state.completed_segments(), 1);
    assert!(state.completed_segments() >= 1);
}

#[test]
fn segment_ids_and_titles_are_stable() {
    let a = FakeSegment::new("alpha", "Alpha", false);
    let b = FakeSegment::new("beta", "Beta", false);
    assert_eq!(a.id(), "alpha");
    assert_eq!(a.title(), "Alpha");
    assert_eq!(b.id(), "beta");
    assert_ne!(a.id(), b.id());
}

#[test]
fn read_yes_no_requires_confirm_to_resolve() {
    use what_you_shouldnt_worry_about_in_2026::segments::read_yes_no;

    let mut yes = ScriptedInput::new(&[InputCommand::Character('y'), InputCommand::Confirm]);
    assert_eq!(read_yes_no(&mut yes).unwrap(), ReadOutcome::Value(true));

    let mut no = ScriptedInput::new(&[InputCommand::Character('n'), InputCommand::Confirm]);
    assert_eq!(read_yes_no(&mut no).unwrap(), ReadOutcome::Value(false));

    let mut quit = ScriptedInput::new(&[InputCommand::Quit]);
    assert_eq!(read_yes_no(&mut quit).unwrap(), ReadOutcome::Quit);
}

#[test]
fn read_numeric_returns_value_or_cancel() {
    use what_you_shouldnt_worry_about_in_2026::segments::read_numeric;

    let mut src = ScriptedInput::new(&[InputCommand::Character('5'), InputCommand::Confirm]);
    assert_eq!(
        read_numeric(&mut src, "Floor: ").unwrap(),
        ReadOutcome::Value(5)
    );

    let mut cancel = ScriptedInput::new(&[InputCommand::Cancel]);
    assert_eq!(
        read_numeric(&mut cancel, "Floor: ").unwrap(),
        ReadOutcome::Cancel
    );
}
