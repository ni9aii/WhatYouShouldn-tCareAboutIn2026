use std::io;

use crate::input::ReadOutcome;
use crate::segments::{Segment, SegmentOutcome, read_yes_no};
use crate::state::GameState;

/// Basic mirror segment for MVP-1: decide whether to look into the mirror.
///
/// The full grid/reflection mini-game is deferred to MVP-2; here the choice
/// nudges the profile per the TZ (look -> esoteric/anxiety, avoid -> locus).
pub struct MirrorSegment;

impl Default for MirrorSegment {
    fn default() -> Self {
        Self
    }
}

impl Segment for MirrorSegment {
    fn id(&self) -> &'static str {
        "mirror"
    }

    fn title(&self) -> &'static str {
        "Mirror Corridor"
    }

    fn run(
        &mut self,
        state: &mut GameState,
        input: &mut dyn crate::input::InputSource,
    ) -> io::Result<SegmentOutcome> {
        if state.is_segment_completed(self.id()) {
            print!("MIRROR SEGMENT\r\n\r\nThis segment is already complete.\r\n");
            return Ok(SegmentOutcome::Completed);
        }
        print!("MIRROR SEGMENT\r\n\r\nA mirror appears in the corridor.\r\n");
        let look = match read_yes_no(input)? {
            ReadOutcome::Value(look) => look,
            ReadOutcome::Cancel => return Ok(SegmentOutcome::Cancelled),
            ReadOutcome::Quit => return Ok(SegmentOutcome::Quit),
        };

        let feedback = state.apply_mirror_decision(look);
        state.complete_segment("mirror");
        print!("\r\n{feedback}\r\n");

        Ok(SegmentOutcome::Completed)
    }
}
