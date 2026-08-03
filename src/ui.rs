//! Terminal presentation helpers.
//!
//! MVP-1 keeps the gameplay output as plain text (the title screen uses
//! ratatui). These helpers centralise onboarding and menu copy so the wording
//! stays consistent and is unit-testable without a terminal.

/// Print the onboarding screen shown before the first segment.
///
/// Explains controls, Esc behaviour, progress persistence and the verdict
/// threshold per the TZ (sections 198-208).
pub fn render_onboarding() {
    let text = onboarding_text();
    print!("{text}");
    use std::io::Write;
    let _ = std::io::stdout().flush();
}

/// The onboarding copy, returned as a string for testing.
pub fn onboarding_text() -> String {
    "\
ONBOARDING — AURA 2026 BROADCAST
================================
You are a viewer-patient on the show. Each segment is a mini-game that quietly
measures your profile. There is no losing: every choice changes the verdict.

CONTROLS
- Arrow keys / number keys: move and choose in menus.
- Enter: confirm a choice or a typed value.
- y / n then Enter: answer yes / no prompts.
- Esc: return to the menu without losing progress.
- q: quit the game.

PROGRESS
- Your profile and completed segments are kept while you play.
- The Oracle verdict unlocks after you complete three segments.
- Replaying a segment does not re-award its profile changes.

Press Enter to enter the broadcast.\r\n"
        .to_owned()
}

/// Format the segment menu for display.
///
/// `completed` holds the ids of segments already finished; they are marked.
pub fn format_menu(
    segments: &[(&str, &str)],
    completed: &std::collections::BTreeSet<String>,
) -> String {
    let mut out =
        String::from("SEGMENT MENU\r\nChoose a segment (press the number or arrow + Enter):\r\n");
    for (index, (id, title)) in segments.iter().enumerate() {
        let mark = if completed.contains(*id) {
            " [done]"
        } else {
            ""
        };
        out.push_str(&format!("{}. {} ({}){}\r\n", index + 1, title, id, mark));
    }
    out.push_str("Press Enter to start the highlighted segment. Esc returns to the menu.\r\n");
    out
}
