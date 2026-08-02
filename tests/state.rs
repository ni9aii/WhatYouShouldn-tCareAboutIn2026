use what_you_shouldnt_care_about_in_2026::state::PlayerAspects;

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
