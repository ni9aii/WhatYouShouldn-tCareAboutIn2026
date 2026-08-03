use what_you_shouldnt_care_about_in_2026::oracle::{self, Axis};
use what_you_shouldnt_care_about_in_2026::state::PlayerAspects;

fn profile_with(mut f: impl FnMut(&mut PlayerAspects)) -> PlayerAspects {
    let mut p = PlayerAspects::default();
    f(&mut p);
    p
}

#[test]
fn dominant_axis_resolves_each_axis() {
    assert_eq!(
        oracle::dominant_axis(&profile_with(|p| p.social_engagement = 90)).0,
        Axis::Social
    );
    assert_eq!(
        oracle::dominant_axis(&profile_with(|p| p.esoteric_attunement = 90)).0,
        Axis::Esoteric
    );
    assert_eq!(
        oracle::dominant_axis(&profile_with(|p| p.geopolitical_awareness = 90)).0,
        Axis::Geopolitical
    );
    assert_eq!(
        oracle::dominant_axis(&profile_with(|p| p.locus_of_control = 90)).0,
        Axis::Locus
    );
}

#[test]
fn classify_detects_markers() {
    let c = oracle::classify(&profile_with(|p| {
        p.baseline_anxiety = 80;
        p.contract_signed = true;
        p.constellation_name = Some("X".to_owned());
        p.keywords.push("echo".to_owned());
    }));
    assert!(c.anxiety_high);
    assert!(c.contract_signed);
    assert!(c.has_constellation);
    assert!(c.has_keywords);
}

#[test]
fn anxiety_threshold_is_strictly_above_70() {
    assert!(!oracle::classify(&profile_with(|p| p.baseline_anxiety = 70)).anxiety_high);
    assert!(oracle::classify(&profile_with(|p| p.baseline_anxiety = 71)).anxiety_high);
}

#[test]
fn neutral_profile_uses_fallback_template() {
    let verdict = oracle::generate(&PlayerAspects::default());
    assert!(verdict.contains("ORACLE VERDICT"));
    assert!(!verdict.contains("{"));
    assert!(verdict.contains("nothing"));
}

#[test]
fn every_template_is_reachable_and_placeholder_free() {
    // Cover the marker combinations that select distinct templates.
    let cases = [
        profile_with(|p| p.social_engagement = 80),
        profile_with(|p| {
            p.social_engagement = 80;
            p.baseline_anxiety = 80;
        }),
        profile_with(|p| {
            p.social_engagement = 80;
            p.contract_signed = true;
        }),
        profile_with(|p| {
            p.social_engagement = 80;
            p.constellation_name = Some("C".to_owned());
        }),
        profile_with(|p| {
            p.social_engagement = 80;
            p.keywords.push("k".to_owned());
        }),
        profile_with(|p| {
            p.social_engagement = 80;
            p.baseline_anxiety = 80;
            p.contract_signed = true;
            p.constellation_name = Some("C".to_owned());
            p.keywords.push("k".to_owned());
        }),
        profile_with(|p| p.esoteric_attunement = 80),
        profile_with(|p| {
            p.esoteric_attunement = 80;
            p.baseline_anxiety = 80;
        }),
        profile_with(|p| {
            p.esoteric_attunement = 80;
            p.contract_signed = true;
        }),
        profile_with(|p| {
            p.esoteric_attunement = 80;
            p.constellation_name = Some("C".to_owned());
        }),
        profile_with(|p| {
            p.esoteric_attunement = 80;
            p.keywords.push("k".to_owned());
        }),
        profile_with(|p| {
            p.esoteric_attunement = 80;
            p.baseline_anxiety = 80;
            p.contract_signed = true;
            p.constellation_name = Some("C".to_owned());
            p.keywords.push("k".to_owned());
        }),
        profile_with(|p| p.geopolitical_awareness = 80),
        profile_with(|p| {
            p.geopolitical_awareness = 80;
            p.baseline_anxiety = 80;
        }),
        profile_with(|p| {
            p.geopolitical_awareness = 80;
            p.contract_signed = true;
        }),
        profile_with(|p| {
            p.geopolitical_awareness = 80;
            p.constellation_name = Some("C".to_owned());
        }),
        profile_with(|p| {
            p.geopolitical_awareness = 80;
            p.keywords.push("k".to_owned());
        }),
        profile_with(|p| {
            p.geopolitical_awareness = 80;
            p.baseline_anxiety = 80;
            p.contract_signed = true;
            p.constellation_name = Some("C".to_owned());
            p.keywords.push("k".to_owned());
        }),
        profile_with(|p| p.locus_of_control = 80),
        profile_with(|p| {
            p.locus_of_control = 80;
            p.baseline_anxiety = 80;
        }),
        profile_with(|p| {
            p.locus_of_control = 80;
            p.contract_signed = true;
        }),
        profile_with(|p| {
            p.locus_of_control = 80;
            p.constellation_name = Some("C".to_owned());
        }),
        profile_with(|p| {
            p.locus_of_control = 80;
            p.keywords.push("k".to_owned());
        }),
        profile_with(|p| {
            p.locus_of_control = 80;
            p.baseline_anxiety = 80;
            p.contract_signed = true;
            p.constellation_name = Some("C".to_owned());
            p.keywords.push("k".to_owned());
        }),
        profile_with(|p| p.baseline_anxiety = 80),
        profile_with(|p| p.contract_signed = true),
        profile_with(|p| p.constellation_name = Some("C".to_owned())),
        profile_with(|p| p.keywords.push("k".to_owned())),
        profile_with(|p| {
            p.baseline_anxiety = 80;
            p.contract_signed = true;
        }),
        profile_with(|p| {
            p.social_engagement = 80;
            p.constellation_name = Some("C".to_owned());
            p.keywords.push("k".to_owned());
        }),
        profile_with(|p| {
            p.esoteric_attunement = 80;
            p.contract_signed = true;
            p.constellation_name = Some("C".to_owned());
        }),
        profile_with(|p| {
            p.geopolitical_awareness = 80;
            p.contract_signed = true;
            p.keywords.push("k".to_owned());
        }),
        profile_with(|p| {
            p.locus_of_control = 80;
            p.constellation_name = Some("C".to_owned());
            p.keywords.push("k".to_owned());
        }),
    ];

    for (i, profile) in cases.iter().enumerate() {
        let verdict = oracle::generate(profile);
        assert!(
            !verdict.contains('{'),
            "case {i} left a placeholder: {verdict}"
        );
        assert!(verdict.starts_with("ORACLE VERDICT"));
        assert!(verdict.len() > 40, "case {i} verdict too short");
    }
    assert!(cases.len() >= 30, "expected >=30 distinct profile cases");
}

#[test]
fn identical_profile_yields_identical_verdict() {
    let make = || {
        profile_with(|p| {
            p.social_engagement = 60;
            p.esoteric_attunement = 40;
            p.baseline_anxiety = 75;
            p.keywords.push("signal".to_owned());
        })
    };
    assert_eq!(oracle::generate(&make()), oracle::generate(&make()));
}

#[test]
fn extreme_profiles_do_not_panic() {
    let high = profile_with(|p| {
        p.social_engagement = 100;
        p.esoteric_attunement = 100;
        p.geopolitical_awareness = 100;
        p.locus_of_control = 100;
        p.baseline_anxiety = 100;
        p.shadow_score = 100;
        p.contract_signed = true;
        p.constellation_name = Some("Omega".to_owned());
        p.keywords.push("all".to_owned());
    });
    let verdict = oracle::generate(&high);
    assert!(!verdict.contains('{'));

    let low = profile_with(|p| {
        p.social_engagement = 0;
        p.esoteric_attunement = 0;
        p.geopolitical_awareness = 0;
        p.locus_of_control = 0;
        p.baseline_anxiety = 0;
    });
    assert!(!oracle::generate(&low).contains('{'));
}

#[test]
fn keyword_inserted_safely_without_braces() {
    let profile = profile_with(|p| p.keywords.push("broadcast".to_owned()));
    let verdict = oracle::generate(&profile);
    assert!(verdict.contains("broadcast"));
    assert!(!verdict.contains("{keyword}"));
}
