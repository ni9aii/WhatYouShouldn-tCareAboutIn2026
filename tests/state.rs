use what_you_shouldnt_care_about_in_2026::state::{GameState, PlayerAspects};

#[test]
fn default_state_uses_default_seed() {
    let state = GameState::default();
    assert_eq!(
        state.seed,
        what_you_shouldnt_care_about_in_2026::state::DEFAULT_SEED
    );
}

#[test]
fn with_seed_stores_seed_and_rng_is_reproducible() {
    let a = GameState::with_seed(42);
    let b = GameState::with_seed(42);
    let c = GameState::with_seed(7);

    use rand::RngCore;
    let mut ra = a.rng();
    let a1 = ra.next_u32();
    let a2 = ra.next_u32();
    let mut rb = b.rng();
    let b1 = rb.next_u32();
    let mut rc = c.rng();
    let c1 = rc.next_u32();

    assert_eq!(a1, b1, "same seed must yield same first draw");
    assert_ne!(a1, c1, "different seed must yield different draw");
    assert_ne!(a1, a2, "sequential draws differ");
}

#[test]
fn profile_updates_are_clamped_to_zero_and_one_hundred() {
    let mut profile = PlayerAspects::default();
    profile.adjust_anxiety(200);
    assert_eq!(profile.baseline_anxiety, 100);

    profile.adjust_anxiety(-300);
    assert_eq!(profile.baseline_anxiety, 0);
}

#[test]
fn keywords_are_unique_and_empty_keywords_are_ignored() {
    let mut profile = PlayerAspects::default();
    profile.add_keyword("");
    profile.add_keyword("static");
    profile.add_keyword("static");

    assert_eq!(profile.keywords, vec!["static"]);
}
