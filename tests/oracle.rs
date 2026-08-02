use what_you_shouldnt_care_about_in_2026::{oracle, state::PlayerAspects};

#[test]
fn oracle_mentions_contract_when_signed() {
    let profile = PlayerAspects {
        contract_signed: true,
        ..Default::default()
    };

    assert!(oracle::generate(&profile).contains("Shadow Corporation"));
}

#[test]
fn oracle_handles_high_anxiety_and_constellation() {
    let profile = PlayerAspects {
        baseline_anxiety: 80,
        constellation_name: Some("The Broken Spoon".to_owned()),
        ..Default::default()
    };

    let verdict = oracle::generate(&profile);
    assert!(verdict.contains("broadcast channel"));
    assert!(verdict.contains("The Broken Spoon"));
}
