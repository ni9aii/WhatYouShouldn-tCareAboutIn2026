use what_you_shouldnt_worry_about_in_2026::state::GameState;

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

#[test]
fn radio_choices_update_different_axes() {
    let mut listening = GameState::default();
    listening.apply_radio_decision(true);
    assert!(listening.profile.geopolitical_awareness > 50);
    assert!(listening.profile.baseline_anxiety > 20);

    let mut silence = GameState::default();
    silence.apply_radio_decision(false);
    assert!(silence.profile.locus_of_control > 50);
    assert!(silence.profile.baseline_anxiety < 20);
}

#[test]
fn mirror_choices_update_different_axes() {
    let mut looking = GameState::default();
    looking.apply_mirror_decision(true);
    assert!(looking.profile.esoteric_attunement > 50);
    assert!(looking.profile.baseline_anxiety > 20);

    let mut avoiding = GameState::default();
    avoiding.apply_mirror_decision(false);
    assert!(avoiding.profile.locus_of_control > 50);
    assert!(avoiding.profile.baseline_anxiety < 20);
}
