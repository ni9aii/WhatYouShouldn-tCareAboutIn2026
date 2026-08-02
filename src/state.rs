use std::collections::BTreeSet;

pub const VERDICT_SEGMENT_THRESHOLD: usize = 3;

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

#[derive(Debug, Default)]
pub struct GameState {
    pub profile: PlayerAspects,
    completed: BTreeSet<String>,
}

impl GameState {
    pub fn completed_segments(&self) -> usize {
        self.completed.len()
    }

    pub fn complete_segment(&mut self, id: &str) {
        self.completed.insert(id.to_owned());
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
}

fn clamp(value: u32, delta: i32) -> u32 {
    (value as i64 + delta as i64).clamp(0, 100) as u32
}
