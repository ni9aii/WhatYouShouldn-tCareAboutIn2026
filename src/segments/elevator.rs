use std::io;

use crate::input::ReadOutcome;
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
        if state.is_segment_completed(self.id()) {
            print!("ELEVATOR SEGMENT\r\n\r\nThis segment is already complete.\r\n");
            return Ok(SegmentOutcome::Completed);
        }
        let snapshot = state.clone();
        print!("ELEVATOR SEGMENT\r\n\r\nChoose a floor from 1 to 100.\r\n");
        let floor = match read_numeric(input, "Elevator is waiting. Type a floor (1-100): ")? {
            ReadOutcome::Value(floor) => floor,
            ReadOutcome::Cancel => return Ok(SegmentOutcome::Cancelled),
            ReadOutcome::Quit => return Ok(SegmentOutcome::Quit),
        };

        print!("\r\nThe elevator shudders between floors. Panic? (y/n, then Enter): ");
        let panic = match read_yes_no(input)? {
            ReadOutcome::Value(panic) => panic,
            ReadOutcome::Cancel => {
                *state = snapshot;
                return Ok(SegmentOutcome::Cancelled);
            }
            ReadOutcome::Quit => {
                *state = snapshot;
                return Ok(SegmentOutcome::Quit);
            }
        };

        let feedback = state.apply_elevator_decision(floor, panic);
        print!("\r\n{feedback}\r\n");

        let category = ElevatorEventCategory::from_floor(floor);
        print!(
            "\r\nOn this floor a {} event is unfolding. Engage with it? (y/n, then Enter): ",
            category.label()
        );
        let engage = match read_yes_no(input)? {
            ReadOutcome::Value(engage) => engage,
            ReadOutcome::Cancel => {
                *state = snapshot;
                return Ok(SegmentOutcome::Cancelled);
            }
            ReadOutcome::Quit => {
                *state = snapshot;
                return Ok(SegmentOutcome::Quit);
            }
        };

        let event_feedback = state.apply_elevator_event(category, engage);
        state.complete_segment("elevator");
        print!("\r\n{event_feedback}\r\n");

        Ok(SegmentOutcome::Completed)
    }
}
