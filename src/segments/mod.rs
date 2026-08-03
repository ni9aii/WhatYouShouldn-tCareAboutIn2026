use std::io;

use crate::input::{InputCommand, InputSource};
use crate::state::GameState;

pub mod elevator;
pub mod radio;

/// Result of running a segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentOutcome {
    /// The segment completed and recorded its progress.
    Completed,
    /// The player cancelled (Esc) and returned to the menu; progress is kept.
    Cancelled,
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
/// Returns `None` when the player cancels (Esc) or quits (q), so the caller can
/// abort the segment cleanly.
pub fn read_yes_no(input: &mut dyn InputSource) -> io::Result<Option<bool>> {
    use crate::input::Choice;
    match crate::input::prompt_choice(input)? {
        Choice::Confirmed => Ok(Some(true)),
        Choice::Cancelled => Ok(Some(false)),
        Choice::Quit => Ok(None),
    }
}

/// Convenience helper for segments: read a numeric choice (e.g. floor).
///
/// Returns `None` when the player cancels (Esc) or quits (q).
pub fn read_numeric(input: &mut dyn InputSource, prompt: &str) -> io::Result<Option<u32>> {
    use crate::input::ReadOutcome;
    match crate::input::read_number(input, prompt)? {
        ReadOutcome::Value(value) => Ok(Some(value)),
        ReadOutcome::Cancel | ReadOutcome::Quit => Ok(None),
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
