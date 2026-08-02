use what_you_shouldnt_care_about_in_2026::state::GameState;

#[test]
fn high_floor_increases_social_and_geopolitical_axes() {
    let mut state = GameState::default();
    let before = state.profile.clone();

    state.apply_elevator_decision(90, false);

    assert!(state.profile.social_engagement > before.social_engagement);
    assert!(state.profile.geopolitical_awareness > before.geopolitical_awareness);
}

#[test]
fn panic_increases_anxiety() {
    let mut state = GameState::default();
    let before = state.profile.baseline_anxiety;

    state.apply_elevator_decision(50, true);

    assert!(state.profile.baseline_anxiety > before);
}
