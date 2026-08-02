use crate::state::PlayerAspects;

pub fn generate(profile: &PlayerAspects) -> String {
    let (axis, value) = dominant_axis(profile);
    let concern = match axis {
        "social" => "other people's opinions",
        "esoteric" => "the collapse of ordinary reality",
        "geopolitical" => "geopolitical consequences",
        _ => "the illusion of control",
    };

    let marker = profile
        .constellation_name
        .as_deref()
        .or_else(|| profile.keywords.first().map(String::as_str))
        .unwrap_or("absolutely nothing");

    let anxiety_line = if profile.baseline_anxiety > 70 {
        "Your anxiety has requested its own broadcast channel."
    } else {
        "Your anxiety remains within commercially acceptable limits."
    };

    let contract_line = if profile.contract_signed {
        "The Shadow Corporation has already processed your consent."
    } else {
        "No shadow contract was signed, which is suspiciously responsible."
    };

    format!(
        "ORACLE VERDICT\n\nYour dominant aura is {axis} ({value}/100).\nYour main concern in 2026 is {concern}.\n\n{anxiety_line} {contract_line}\n\nRemember {marker}: everything is temporary, especially your certainty."
    )
}

fn dominant_axis(profile: &PlayerAspects) -> (&'static str, u32) {
    [
        ("social", profile.social_engagement),
        ("esoteric", profile.esoteric_attunement),
        ("geopolitical", profile.geopolitical_awareness),
        ("locus", profile.locus_of_control),
    ]
    .into_iter()
    .max_by_key(|(_, value)| *value)
    .unwrap_or(("locus", 0))
}
