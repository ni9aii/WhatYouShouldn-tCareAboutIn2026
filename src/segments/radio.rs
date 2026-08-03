use std::io;

use crate::segments::{Segment, SegmentOutcome, read_yes_no};
use crate::state::GameState;

/// Basic radio segment for MVP-1: decide whether to listen, and if so, catch a
/// keyword for the player profile.
///
/// The full frequency-tuning mini-game (0..=100, capture 3 words) is deferred
/// to MVP-2; here a single caught keyword is recorded when the player listens.
pub struct RadioSegment;

impl Default for RadioSegment {
    fn default() -> Self {
        Self
    }
}

impl Segment for RadioSegment {
    fn id(&self) -> &'static str {
        "radio"
    }

    fn title(&self) -> &'static str {
        "Radio Receiver"
    }

    fn run(
        &mut self,
        state: &mut GameState,
        input: &mut dyn crate::input::InputSource,
    ) -> io::Result<SegmentOutcome> {
        print!("RADIO SEGMENT\r\n\r\nThe radio starts broadcasting.\r\n");
        let listen = match read_yes_no(input)? {
            Some(listen) => listen,
            None => return Ok(SegmentOutcome::Cancelled),
        };

        let feedback = state.apply_radio_decision(listen);
        state.complete_segment("radio");
        print!("\r\n{feedback}\r\n");

        if listen {
            let keyword = "broadcast";
            state.profile.add_keyword(keyword);
            print!("\r\nYou caught a keyword: \"{keyword}\".\r\n");
        }

        Ok(SegmentOutcome::Completed)
    }
}
