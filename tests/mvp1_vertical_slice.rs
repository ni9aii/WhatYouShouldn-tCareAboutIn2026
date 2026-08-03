use what_you_shouldnt_worry_about_in_2026::{oracle, state::GameState};

#[test]
fn default_game_state_starts_empty_and_cannot_show_verdict() {
    let state = GameState::default();

    assert_eq!(state.completed_segments(), 0);
    assert!(!state.can_show_verdict());
}

#[test]
fn three_completed_segments_unlock_the_verdict() {
    let mut state = GameState::default();

    state.complete_segment("elevator");
    state.complete_segment("radio");
    state.complete_segment("mirror");

    assert_eq!(state.completed_segments(), 3);
    assert!(state.can_show_verdict());
}

#[test]
fn elevator_decision_changes_profile_and_returns_feedback() {
    let mut state = GameState::default();
    let feedback = state.apply_elevator_decision(90, false);

    assert!(feedback.contains("elevator"));
    assert!(state.profile.geopolitical_awareness > 0);
    assert!(state.profile.social_engagement > 0);
}

#[test]
fn oracle_returns_a_deterministic_english_verdict() {
    let state = GameState::default();
    let first = oracle::generate(&state.profile);
    let second = oracle::generate(&state.profile);

    assert_eq!(first, second);
    assert!(!first.trim().is_empty());
    assert!(first.is_ascii());
}
