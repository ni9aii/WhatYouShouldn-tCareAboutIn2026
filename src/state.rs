use std::collections::BTreeSet;

use rand::SeedableRng;
use rand::rngs::StdRng;

pub const VERDICT_SEGMENT_THRESHOLD: usize = 3;

/// Default seed used when no explicit seed is provided. Tests pass a fixed
/// seed so runs stay reproducible.
pub const DEFAULT_SEED: u64 = 0x2026_C0DE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerAspects {
    pub social_engagement: u32,
    pub esoteric_attunement: u32,
    pub geopolitical_awareness: u32,
    pub locus_of_control: u32,
    pub baseline_anxiety: u32,
    pub shadow_score: u32,
    pub contract_signed: bool,
    pub constellation_name: Option<String>,
    pub keywords: Vec<String>,
}

impl Default for PlayerAspects {
    fn default() -> Self {
        Self {
            social_engagement: 50,
            esoteric_attunement: 50,
            geopolitical_awareness: 50,
            locus_of_control: 50,
            baseline_anxiety: 20,
            shadow_score: 0,
            contract_signed: false,
            constellation_name: None,
            keywords: Vec::new(),
        }
    }
}

impl PlayerAspects {
    pub fn adjust_social(&mut self, delta: i32) {
        self.social_engagement = clamp(self.social_engagement, delta);
    }

    pub fn adjust_esoteric(&mut self, delta: i32) {
        self.esoteric_attunement = clamp(self.esoteric_attunement, delta);
    }

    pub fn adjust_geopolitical(&mut self, delta: i32) {
        self.geopolitical_awareness = clamp(self.geopolitical_awareness, delta);
    }

    pub fn adjust_locus(&mut self, delta: i32) {
        self.locus_of_control = clamp(self.locus_of_control, delta);
    }

    pub fn adjust_anxiety(&mut self, delta: i32) {
        self.baseline_anxiety = clamp(self.baseline_anxiety, delta);
    }

    pub fn adjust_shadow(&mut self, delta: i32) {
        self.shadow_score = clamp(self.shadow_score, delta);
    }

    pub fn add_keyword(&mut self, keyword: impl Into<String>) {
        let keyword = keyword.into();
        if !keyword.is_empty() && !self.keywords.iter().any(|item| item == &keyword) {
            self.keywords.push(keyword);
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub profile: PlayerAspects,
    completed: BTreeSet<String>,
    pub seed: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            profile: PlayerAspects::default(),
            completed: BTreeSet::new(),
            seed: DEFAULT_SEED,
        }
    }
}

impl GameState {
    /// Create a `GameState` with a fixed seed for reproducible runs.
    pub fn with_seed(seed: u64) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }

    /// Build a deterministic RNG from the stored seed.
    ///
    /// Segments and the oracle use this for any randomness so that a fixed
    /// seed yields identical behaviour in tests and replays.
    pub fn rng(&self) -> StdRng {
        StdRng::seed_from_u64(self.seed)
    }

    pub fn completed_segments(&self) -> usize {
        self.completed.len()
    }

    /// Borrow the set of completed segment ids (for menu rendering).
    pub fn completed_set(&self) -> &BTreeSet<String> {
        &self.completed
    }

    pub fn complete_segment(&mut self, id: &str) {
        self.completed.insert(id.to_owned());
    }

    pub fn is_segment_completed(&self, id: &str) -> bool {
        self.completed.contains(id)
    }

    pub fn can_show_verdict(&self) -> bool {
        self.completed_segments() >= VERDICT_SEGMENT_THRESHOLD
    }

    pub fn apply_elevator_decision(&mut self, floor: u32, panic: bool) -> String {
        if floor >= 70 {
            self.profile.adjust_social(8);
            self.profile.adjust_geopolitical(8);
        } else {
            self.profile.adjust_esoteric(8);
        }

        if panic {
            self.profile.adjust_anxiety(10);
            self.profile.adjust_locus(5);
            "The elevator stops. Your panic has been filed under: elevator.".to_owned()
        } else {
            self.profile.adjust_anxiety(-5);
            self.profile.adjust_esoteric(3);
            "The elevator arrives. The building has noticed your composure.".to_owned()
        }
    }

    pub fn apply_radio_decision(&mut self, listen: bool) -> String {
        if listen {
            self.profile.adjust_geopolitical(6);
            self.profile.adjust_anxiety(4);
            "The radio explains everything, except why you are still listening.".to_owned()
        } else {
            self.profile.adjust_locus(6);
            self.profile.adjust_anxiety(-3);
            "You turn off the radio. The silence has its own editorial position.".to_owned()
        }
    }

    pub fn apply_mirror_decision(&mut self, look: bool) -> String {
        if look {
            self.profile.adjust_esoteric(6);
            self.profile.adjust_anxiety(5);
            "The mirror confirms your presence and declines to elaborate.".to_owned()
        } else {
            self.profile.adjust_locus(5);
            self.profile.adjust_anxiety(-4);
            "You avoid the mirror. Nothing objectively important has changed.".to_owned()
        }
    }
}

/// Event category that can occur on an elevator floor, per the TZ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevatorEventCategory {
    Mystical,
    Social,
    Global,
}

impl ElevatorEventCategory {
    /// Pick a deterministic event category from a chosen floor (1..=100).
    ///
    /// High floors read as geopolitical, mid floors as social, low floors as
    /// esoteric. The mapping is fixed so tests stay reproducible.
    pub fn from_floor(floor: u32) -> Self {
        if floor >= 70 {
            Self::Global
        } else if floor >= 40 {
            Self::Social
        } else {
            Self::Mystical
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Mystical => "mystical",
            Self::Social => "social",
            Self::Global => "global",
        }
    }
}

impl GameState {
    pub fn apply_elevator_event(
        &mut self,
        category: ElevatorEventCategory,
        engage: bool,
    ) -> String {
        if !engage {
            return "You ignore the floor's event. The doors close on an unremarkable moment."
                .to_owned();
        }
        match category {
            ElevatorEventCategory::Mystical => {
                self.profile.adjust_esoteric(6);
                "A mystical event unfolds: the elevator hums a tune older than the building."
                    .to_owned()
            }
            ElevatorEventCategory::Social => {
                self.profile.adjust_social(6);
                "A social event unfolds: a neighbour nods, and you are briefly legible to society."
                    .to_owned()
            }
            ElevatorEventCategory::Global => {
                self.profile.adjust_geopolitical(6);
                "A global event unfolds: the rooftop broadcast declares a crisis that will not involve you.".to_owned()
            }
        }
    }
}

fn clamp(value: u32, delta: i32) -> u32 {
    (value as i64 + delta as i64).clamp(0, 100) as u32
}
