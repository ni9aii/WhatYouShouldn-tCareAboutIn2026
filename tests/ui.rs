//! Tests for the pure UI text helpers (no terminal required).

use std::collections::BTreeSet;

use what_you_shouldnt_worry_about_in_2026::ui::{format_menu, onboarding_text};

#[test]
fn onboarding_mentions_controls_and_verdict_threshold() {
    let text = onboarding_text();
    assert!(text.contains("Esc"));
    assert!(text.contains("Enter"));
    assert!(text.contains("three segments"));
    assert!(text.contains("without losing progress"));
}

#[test]
fn menu_marks_completed_segments() {
    let mut done = BTreeSet::new();
    done.insert("elevator".to_owned());
    let segments = [
        ("elevator", "Elevator in the Skyscraper"),
        ("radio", "Radio Receiver"),
        ("mirror", "Mirror Corridor"),
    ];
    let menu = format_menu(&segments, &done, 0, false);
    assert!(menu.contains("elevator"));
    assert!(menu.contains("[done]"));
    assert!(menu.contains("radio"));
    assert!(!menu.contains("Radio Receiver [done]"));
}
