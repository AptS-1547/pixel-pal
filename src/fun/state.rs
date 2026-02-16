use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ComboTracker {
    streak: u8,
    last_click_secs: Option<f64>,
}

impl ComboTracker {
    const COMBO_WINDOW_SECS: f64 = 0.65;

    pub fn register_click(&mut self, now_secs: f64) -> bool {
        let within_window = self
            .last_click_secs
            .is_some_and(|last| now_secs - last <= Self::COMBO_WINDOW_SECS);

        if within_window {
            self.streak = self.streak.saturating_add(1);
        } else {
            self.streak = 1;
        }

        self.last_click_secs = Some(now_secs);

        if self.streak >= 3 {
            self.streak = 0;
            self.last_click_secs = None;
            true
        } else {
            false
        }
    }
}

#[derive(Resource)]
pub struct ReactionGame {
    pub(crate) state: ReactionState,
}

impl Default for ReactionGame {
    fn default() -> Self {
        Self {
            state: ReactionState::Idle,
        }
    }
}

impl ReactionGame {
    pub fn is_active(&self) -> bool {
        !matches!(self.state, ReactionState::Idle)
    }
}

pub(crate) enum ReactionState {
    Idle,
    Waiting { timer: Timer },
    Go { elapsed_ms: f32, timeout: Timer },
    Cooldown { timer: Timer },
}

#[derive(Resource)]
pub struct FunHudMessage {
    pub text: String,
    pub visible: bool,
    pub(crate) timer: Timer,
}

impl Default for FunHudMessage {
    fn default() -> Self {
        Self {
            text: "Press R to start reaction game".to_string(),
            visible: true,
            timer: Timer::from_seconds(4.0, TimerMode::Once),
        }
    }
}

#[derive(Resource, Default)]
pub struct FunProgress {
    pub(crate) feed_count: u32,
    pub(crate) pet_count: u32,
    pub(crate) combo_count: u32,
    pub(crate) reaction_runs: u32,
    pub(crate) best_reaction_ms: Option<u32>,
    pub(crate) unlocked: HashSet<Achievement>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Achievement {
    FirstFeed,
    PetLover,
    ComboStarter,
    ReflexAce,
}

impl Achievement {
    pub(crate) fn title(self) -> &'static str {
        match self {
            Achievement::FirstFeed => "Unlocked: First Feed",
            Achievement::PetLover => "Unlocked: Pet Lover",
            Achievement::ComboStarter => "Unlocked: Combo Starter",
            Achievement::ReflexAce => "Unlocked: Reflex Ace",
        }
    }
}
