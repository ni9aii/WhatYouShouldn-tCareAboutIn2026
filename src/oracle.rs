use crate::state::PlayerAspects;

/// Classification of a profile used to select a verdict template.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Classification {
    pub axis: Axis,
    pub anxiety_high: bool,
    pub contract_signed: bool,
    pub has_constellation: bool,
    pub has_keywords: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Social,
    Esoteric,
    Geopolitical,
    Locus,
}

impl Axis {
    pub fn name(self) -> &'static str {
        match self {
            Axis::Social => "social",
            Axis::Esoteric => "esoteric",
            Axis::Geopolitical => "geopolitical",
            Axis::Locus => "locus",
        }
    }

    pub fn concern(self) -> &'static str {
        match self {
            Axis::Social => "other people's opinions",
            Axis::Esoteric => "the collapse of ordinary reality",
            Axis::Geopolitical => "geopolitical consequences",
            Axis::Locus => "the illusion of control",
        }
    }
}

/// Pick the dominant axis. Ties resolve to the order
/// social > esoteric > geopolitical > locus (deterministic).
pub fn dominant_axis(profile: &PlayerAspects) -> (Axis, u32) {
    let candidates = [
        (Axis::Social, profile.social_engagement),
        (Axis::Esoteric, profile.esoteric_attunement),
        (Axis::Geopolitical, profile.geopolitical_awareness),
        (Axis::Locus, profile.locus_of_control),
    ];
    let (axis, value) = candidates
        .into_iter()
        .max_by_key(|(_, value)| *value)
        .unwrap_or((Axis::Locus, 0));
    (axis, value)
}

/// Classify a profile for verdict selection.
pub fn classify(profile: &PlayerAspects) -> Classification {
    let (axis, _) = dominant_axis(profile);
    Classification {
        axis,
        anxiety_high: profile.baseline_anxiety > 70,
        contract_signed: profile.contract_signed,
        has_constellation: profile.constellation_name.is_some(),
        has_keywords: !profile.keywords.is_empty(),
    }
}

/// A single verdict template with slots filled from the profile.
struct Template {
    axis: Option<Axis>,
    anxiety_high: bool,
    contract: bool,
    constellation: bool,
    keywords: bool,
    text: &'static str,
}

impl Template {
    fn matches(&self, c: &Classification) -> bool {
        if self.axis.is_some_and(|axis| axis != c.axis) {
            return false;
        }
        if self.anxiety_high != c.anxiety_high {
            return false;
        }
        if self.contract != c.contract_signed {
            return false;
        }
        if self.constellation != c.has_constellation {
            return false;
        }
        if self.keywords != c.has_keywords {
            return false;
        }
        true
    }
}

const TEMPLATES: &[Template] = &[
    // --- Social-dominant ---
    Template {
        axis: Some(Axis::Social),
        anxiety_high: false,
        contract: false,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is social. In 2026 your main concern is other people's opinions, which is efficient: you can ignore reality and still trend.",
    },
    Template {
        axis: Some(Axis::Social),
        anxiety_high: true,
        contract: false,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is social and your anxiety has its own publicist. In 2026 your main concern is other people's opinions, broadcast live and unsponsored.",
    },
    Template {
        axis: Some(Axis::Social),
        anxiety_high: false,
        contract: true,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is social. The Shadow Corporation has already processed your consent, and frankly you signed faster than you like a post.",
    },
    Template {
        axis: Some(Axis::Social),
        anxiety_high: false,
        contract: false,
        constellation: true,
        keywords: false,
        text: "Your dominant aura is social. You drew the constellation of the Crowd, and the Crowd has noticed you noticing it.",
    },
    Template {
        axis: Some(Axis::Social),
        anxiety_high: false,
        contract: false,
        constellation: false,
        keywords: true,
        text: "Your dominant aura is social. You collected the keyword '{keyword}', which the broadcast will now use to describe you to yourself.",
    },
    Template {
        axis: Some(Axis::Social),
        anxiety_high: true,
        contract: true,
        constellation: true,
        keywords: true,
        text: "Your dominant aura is social, your anxiety is broadcast-grade, and your constellation '{constellation}' was filed under 'influencer'. The contract is signed.",
    },
    // --- Esoteric-dominant ---
    Template {
        axis: Some(Axis::Esoteric),
        anxiety_high: false,
        contract: false,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is esoteric. In 2026 your main concern is the collapse of ordinary reality, which is on schedule and does not require your attendance.",
    },
    Template {
        axis: Some(Axis::Esoteric),
        anxiety_high: true,
        contract: false,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is esoteric and your anxiety hums at a frequency only the radio understands. In 2026 your main concern is the collapse of ordinary reality.",
    },
    Template {
        axis: Some(Axis::Esoteric),
        anxiety_high: false,
        contract: true,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is esoteric. The Shadow Corporation has already processed your consent, which it read in your aura before you arrived.",
    },
    Template {
        axis: Some(Axis::Esoteric),
        anxiety_high: false,
        contract: false,
        constellation: true,
        keywords: false,
        text: "Your dominant aura is esoteric. You drew the constellation '{constellation}', a shape the Oracle has seen exactly once and chosen not to explain.",
    },
    Template {
        axis: Some(Axis::Esoteric),
        anxiety_high: false,
        contract: false,
        constellation: false,
        keywords: true,
        text: "Your dominant aura is esoteric. You caught the keyword '{keyword}', which resonates with a door you have not opened yet.",
    },
    Template {
        axis: Some(Axis::Esoteric),
        anxiety_high: true,
        contract: true,
        constellation: true,
        keywords: true,
        text: "Your dominant aura is esoteric, your anxiety is cosmic, your constellation '{constellation}' is sealed, and the keyword '{keyword}' is now part of the contract.",
    },
    // --- Geopolitical-dominant ---
    Template {
        axis: Some(Axis::Geopolitical),
        anxiety_high: false,
        contract: false,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is geopolitical. In 2026 your main concern is geopolitical consequences, none of which will involve you personally, by design.",
    },
    Template {
        axis: Some(Axis::Geopolitical),
        anxiety_high: true,
        contract: false,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is geopolitical and your anxiety tracks the news in real time. In 2026 your main concern is geopolitical consequences, buffered by denial.",
    },
    Template {
        axis: Some(Axis::Geopolitical),
        anxiety_high: false,
        contract: true,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is geopolitical. The Shadow Corporation has already processed your consent, which is now a matter of international record.",
    },
    Template {
        axis: Some(Axis::Geopolitical),
        anxiety_high: false,
        contract: false,
        constellation: true,
        keywords: false,
        text: "Your dominant aura is geopolitical. You drew the constellation '{constellation}', a map of borders that do not exist yet.",
    },
    Template {
        axis: Some(Axis::Geopolitical),
        anxiety_high: false,
        contract: false,
        constellation: false,
        keywords: true,
        text: "Your dominant aura is geopolitical. You logged the keyword '{keyword}', which the broadcast will cite as a primary source.",
    },
    Template {
        axis: Some(Axis::Geopolitical),
        anxiety_high: true,
        contract: true,
        constellation: true,
        keywords: true,
        text: "Your dominant aura is geopolitical, your anxiety is diplomatic, your constellation '{constellation}' is classified, and the keyword '{keyword}' is redacted.",
    },
    // --- Locus-dominant ---
    Template {
        axis: Some(Axis::Locus),
        anxiety_high: false,
        contract: false,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is locus. In 2026 your main concern is the illusion of control, which you maintain with admirable discipline.",
    },
    Template {
        axis: Some(Axis::Locus),
        anxiety_high: true,
        contract: false,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is locus and your anxiety is a control panel with no off switch. In 2026 your main concern is the illusion of control.",
    },
    Template {
        axis: Some(Axis::Locus),
        anxiety_high: false,
        contract: true,
        constellation: false,
        keywords: false,
        text: "Your dominant aura is locus. The Shadow Corporation has already processed your consent, which you signed because you prefer to hold the pen.",
    },
    Template {
        axis: Some(Axis::Locus),
        anxiety_high: false,
        contract: false,
        constellation: true,
        keywords: false,
        text: "Your dominant aura is locus. You drew the constellation '{constellation}', a diagram of every lever you believe you pull.",
    },
    Template {
        axis: Some(Axis::Locus),
        anxiety_high: false,
        contract: false,
        constellation: false,
        keywords: true,
        text: "Your dominant aura is locus. You kept the keyword '{keyword}', a small control you can still pretend to own.",
    },
    Template {
        axis: Some(Axis::Locus),
        anxiety_high: true,
        contract: true,
        constellation: true,
        keywords: true,
        text: "Your dominant aura is locus, your anxiety is the fine print, your constellation '{constellation}' is notarised, and the keyword '{keyword}' is yours until revoked.",
    },
    // --- Cross-axis / marker variants (axis-agnostic) ---
    Template {
        axis: None,
        anxiety_high: true,
        contract: false,
        constellation: false,
        keywords: false,
        text: "Your anxiety has requested its own broadcast channel. Whatever your aura, in 2026 it will be accompanied by tasteful static.",
    },
    Template {
        axis: None,
        anxiety_high: false,
        contract: true,
        constellation: false,
        keywords: false,
        text: "The Shadow Corporation thanks you for signing the contract. In 2026, your anxious thoughts will arrive strictly on schedule, and everything else is not your problem.",
    },
    Template {
        axis: None,
        anxiety_high: false,
        contract: false,
        constellation: true,
        keywords: false,
        text: "You named a constellation: '{constellation}'. The Oracle will refer to you by it now, which is cheaper than a surname.",
    },
    Template {
        axis: None,
        anxiety_high: false,
        contract: false,
        constellation: false,
        keywords: true,
        text: "You collected the keyword '{keyword}'. The broadcast will echo it back until it becomes a memory you are certain is yours.",
    },
    Template {
        axis: None,
        anxiety_high: true,
        contract: true,
        constellation: false,
        keywords: false,
        text: "Your anxiety is high and the contract is signed. In 2026 the Shadow Corporation will worry on your behalf, efficiently and without refund.",
    },
    // --- Neutral / fallback ---
    Template {
        axis: None,
        anxiety_high: false,
        contract: false,
        constellation: false,
        keywords: false,
        text: "According to our analysis, your main concern in 2026 is nothing. Geopolitical crises will pass you by because you are too busy drawing constellations that do not exist.",
    },
];

/// Generate a deterministic, English-language Oracle verdict for a profile.
pub fn generate(profile: &PlayerAspects) -> String {
    let classification = classify(profile);
    let (axis, value) = dominant_axis(profile);

    let template = TEMPLATES
        .iter()
        .find(|t| t.matches(&classification))
        .unwrap_or_else(|| {
            // Neutral fallback when no marker-specific template applies.
            TEMPLATES
                .iter()
                .find(|t| {
                    t.axis.is_none()
                        && !t.anxiety_high
                        && !t.contract
                        && !t.constellation
                        && !t.keywords
                })
                .unwrap()
        });

    let marker = profile
        .constellation_name
        .as_deref()
        .or_else(|| profile.keywords.first().map(String::as_str))
        .unwrap_or("absolutely nothing");

    let anxiety_line = if classification.anxiety_high {
        "Your anxiety has requested its own broadcast channel."
    } else {
        "Your anxiety remains within commercially acceptable limits."
    };

    let contract_line = if classification.contract_signed {
        "The Shadow Corporation has already processed your consent."
    } else {
        "No shadow contract was signed, which is suspiciously responsible."
    };

    let text = template
        .text
        .replace("{constellation}", marker)
        .replace("{keyword}", marker);

    format!(
        "ORACLE VERDICT\n\nYour dominant aura is {axis} ({value}/100). Your main concern in 2026 is {concern}.\n\n{body}\n\n{anxiety_line} {contract_line}\n\nRemember {marker}: everything is temporary, especially your certainty.",
        axis = axis.name(),
        value = value,
        concern = axis.concern(),
        body = text,
        anxiety_line = anxiety_line,
        contract_line = contract_line,
        marker = marker,
    )
}
