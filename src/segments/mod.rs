use std::io;

use crate::input::{InputCommand, InputSource, ReadOutcome};
use crate::state::GameState;

pub mod elevator;
pub mod mirror;
pub mod radio;

/// Result of running a segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentOutcome {
    /// The segment completed and recorded its progress.
    Completed,
    /// The player cancelled (Esc) and returned to the menu; progress is kept.
    Cancelled,
    /// The player quit the game from inside the segment.
    Quit,
}

/// A mini-game that mutates the shared `GameState` through abstract input.
///
/// Segments never read `crossterm::Event` directly; they receive an
/// `InputSource` so the same logic can be driven by a scripted source in tests
/// without opening a terminal. A segment records completion only on success.
pub trait Segment {
    /// Stable, unique segment identifier (used for progress tracking).
    fn id(&self) -> &'static str;

    /// Human-readable title shown in menus and on screen.
    fn title(&self) -> &'static str;

    /// Run the segment, mutating `state`. Returns `Cancelled` when the player
    /// backs out so the caller can preserve progress and return to the menu.
    fn run(
        &mut self,
        state: &mut GameState,
        input: &mut dyn InputSource,
    ) -> io::Result<SegmentOutcome>;
}

/// Convenience helper for segments: read a yes/no decision with Enter confirm.
///
/// Returns a distinct cancel or quit outcome so callers can preserve atomic
/// segment behavior and terminate the session when requested.
pub fn read_yes_no(input: &mut dyn InputSource) -> io::Result<ReadOutcome<bool>> {
    use crate::input::Choice;
    match crate::input::prompt_choice(input)? {
        Choice::Selected(value) => Ok(ReadOutcome::Value(value)),
        Choice::Cancelled => Ok(ReadOutcome::Cancel),
        Choice::Quit => Ok(ReadOutcome::Quit),
    }
}

/// Convenience helper for segments: read a numeric choice (e.g. floor).
///
/// Returns a distinct cancel or quit outcome.
pub fn read_numeric(input: &mut dyn InputSource, prompt: &str) -> io::Result<ReadOutcome<u32>> {
    match crate::input::read_number(input, prompt)? {
        ReadOutcome::Value(value) => Ok(ReadOutcome::Value(value)),
        ReadOutcome::Cancel => Ok(ReadOutcome::Cancel),
        ReadOutcome::Quit => Ok(ReadOutcome::Quit),
    }
}

/// Map a keyboard command to a menu navigation intent.
///
/// Used by the segment menu so individual segments stay input-agnostic.
pub fn menu_key_to_index(key: InputCommand, count: usize) -> Option<usize> {
    match key {
        InputCommand::Up => Some(count.wrapping_sub(1)),
        InputCommand::Down => Some(1),
        InputCommand::Confirm => Some(0),
        _ => None,
    }
}

/// Build the MVP-1 segment registry in menu display order.
pub fn all_segments() -> Vec<Box<dyn Segment>> {
    vec![
        Box::new(elevator::ElevatorSegment),
        Box::new(radio::RadioSegment),
        Box::new(mirror::MirrorSegment),
    ]
}

/// Segment id/title pairs for menu rendering.
pub fn segment_entries() -> Vec<(&'static str, &'static str)> {
    all_segments().iter().map(|s| (s.id(), s.title())).collect()
}
