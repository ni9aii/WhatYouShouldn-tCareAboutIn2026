use std::io;

use crate::segments::{Segment, SegmentOutcome, read_numeric, read_yes_no};
use crate::state::{ElevatorEventCategory, GameState};

/// Onboarding segment: choose a floor, decide whether to panic, and engage
/// with the floor's event. Demonstrates the decision→feedback→profile loop.
pub struct ElevatorSegment;

impl Default for ElevatorSegment {
    fn default() -> Self {
        Self
    }
}

impl Segment for ElevatorSegment {
    fn id(&self) -> &'static str {
        "elevator"
    }

    fn title(&self) -> &'static str {
        "Elevator in the Skyscraper"
    }

    fn run(
        &mut self,
        state: &mut GameState,
        input: &mut dyn crate::input::InputSource,
    ) -> io::Result<SegmentOutcome> {
        print!("ELEVATOR SEGMENT\r\n\r\nChoose a floor from 1 to 100.\r\n");
        let floor = match read_numeric(input, "Elevator is waiting. Type a floor (1-100): ")? {
            Some(floor) => floor,
            None => return Ok(SegmentOutcome::Cancelled),
        };

        let panic = match read_yes_no(input)? {
            Some(panic) => panic,
            None => return Ok(SegmentOutcome::Cancelled),
        };
        print!("\r\nThe elevator shudders between floors. Panic? (y/n, then Enter): ");

        let feedback = state.apply_elevator_decision(floor, panic);
        state.complete_segment("elevator");
        print!("\r\n{feedback}\r\n");

        let category = ElevatorEventCategory::from_floor(floor);
        print!(
            "\r\nOn this floor a {} event is unfolding. Engage with it? (y/n, then Enter): ",
            category.label()
        );
        let engage = match read_yes_no(input)? {
            Some(engage) => engage,
            None => return Ok(SegmentOutcome::Cancelled),
        };

        let event_feedback = state.apply_elevator_event(category, engage);
        print!("\r\n{event_feedback}\r\n");

        Ok(SegmentOutcome::Completed)
    }
}
