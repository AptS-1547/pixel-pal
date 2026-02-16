use bevy::prelude::*;

use crate::animation::{ActionStateMachine, UserAction};

use super::events::FunEvent;
use super::state::{Achievement, FunHudMessage, FunProgress, ReactionGame, ReactionState};

pub fn update_reaction_game(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut reaction_game: ResMut<ReactionGame>,
    mut action_state_machine: ResMut<ActionStateMachine>,
    mut fun_events: MessageWriter<FunEvent>,
) {
    try_start_reaction_game(
        &keys,
        &time,
        &mut reaction_game,
        action_state_machine.as_ref(),
        &mut fun_events,
    );
    tick_reaction_state(
        &keys,
        &time,
        &mut reaction_game.state,
        action_state_machine.as_mut(),
        &mut fun_events,
    );
}

pub fn process_fun_events(
    mut events: MessageReader<FunEvent>,
    mut progress: ResMut<FunProgress>,
    mut hud: ResMut<FunHudMessage>,
) {
    for event in events.read() {
        handle_fun_event(event, &mut progress, &mut hud);
        update_achievements(&mut progress, &mut hud);
    }
}

pub fn tick_fun_hud_message(time: Res<Time>, mut hud: ResMut<FunHudMessage>) {
    if hud.visible && hud.timer.tick(time.delta()).just_finished() {
        hud.visible = false;
    }
}

fn try_start_reaction_game(
    keys: &ButtonInput<KeyCode>,
    time: &Time,
    reaction_game: &mut ReactionGame,
    action_state_machine: &ActionStateMachine,
    fun_events: &mut MessageWriter<FunEvent>,
) {
    if !keys.just_pressed(KeyCode::KeyR)
        || reaction_game.is_active()
        || !action_state_machine.is_idle()
    {
        return;
    }

    let delay = reaction_start_delay(time);
    reaction_game.state = ReactionState::Waiting {
        timer: Timer::from_seconds(delay, TimerMode::Once),
    };
    fun_events.write(FunEvent::Notify(
        "Reaction game: wait for GO...".to_string(),
    ));
}

fn tick_reaction_state(
    keys: &ButtonInput<KeyCode>,
    time: &Time,
    state: &mut ReactionState,
    action_state_machine: &mut ActionStateMachine,
    fun_events: &mut MessageWriter<FunEvent>,
) {
    let transition = match state {
        ReactionState::Idle => None,
        ReactionState::Waiting { timer } => evaluate_waiting_state(timer, keys, time, fun_events),
        ReactionState::Go {
            elapsed_ms,
            timeout,
        } => evaluate_go_state(
            elapsed_ms,
            timeout,
            keys,
            time,
            action_state_machine,
            fun_events,
        ),
        ReactionState::Cooldown { timer } => evaluate_cooldown_state(timer, time),
    };

    if let Some(next_state) = transition {
        *state = next_state;
    }
}

fn evaluate_waiting_state(
    timer: &mut Timer,
    keys: &ButtonInput<KeyCode>,
    time: &Time,
    fun_events: &mut MessageWriter<FunEvent>,
) -> Option<ReactionState> {
    if pressed_space(keys) {
        emit_reaction_failure(fun_events, "Too early! Press R to retry");
        return Some(cooldown_state(1.2));
    }

    if timer.tick(time.delta()).just_finished() {
        fun_events.write(FunEvent::Notify("GO! Press SPACE now!".to_string()));
        return Some(go_state());
    }

    None
}

fn evaluate_go_state(
    elapsed_ms: &mut f32,
    timeout: &mut Timer,
    keys: &ButtonInput<KeyCode>,
    time: &Time,
    action_state_machine: &mut ActionStateMachine,
    fun_events: &mut MessageWriter<FunEvent>,
) -> Option<ReactionState> {
    if pressed_space(keys) {
        let reaction_ms = finalize_reaction_ms(*elapsed_ms);
        action_state_machine.trigger(UserAction::Dance);
        emit_reaction_success(fun_events, reaction_ms);
        return Some(cooldown_state(1.5));
    }

    *elapsed_ms += time.delta_secs() * 1000.0;
    if timeout.tick(time.delta()).just_finished() {
        emit_reaction_failure(fun_events, "Missed! Press R to retry");
        return Some(cooldown_state(1.2));
    }

    None
}

fn evaluate_cooldown_state(timer: &mut Timer, time: &Time) -> Option<ReactionState> {
    if timer.tick(time.delta()).just_finished() {
        Some(ReactionState::Idle)
    } else {
        None
    }
}

fn handle_fun_event(event: &FunEvent, progress: &mut FunProgress, hud: &mut FunHudMessage) {
    match event {
        FunEvent::ActionTriggered(action) => record_action(*action, progress),
        FunEvent::ComboTriggered => on_combo(progress, hud),
        FunEvent::ReactionFinished {
            success,
            reaction_ms,
        } => on_reaction_finished(*success, *reaction_ms, progress, hud),
        FunEvent::Notify(message) => show_hud(hud, message.clone(), 2.0),
    }
}

fn record_action(action: UserAction, progress: &mut FunProgress) {
    match action {
        UserAction::Feed => progress.feed_count += 1,
        UserAction::Pet => progress.pet_count += 1,
        _ => {}
    }
}

fn on_combo(progress: &mut FunProgress, hud: &mut FunHudMessage) {
    progress.combo_count += 1;
    show_hud(hud, "Combo x3! Hidden dance activated", 2.0);
}

fn on_reaction_finished(
    success: bool,
    reaction_ms: Option<u32>,
    progress: &mut FunProgress,
    hud: &mut FunHudMessage,
) {
    progress.reaction_runs += 1;

    if success {
        let ms = reaction_ms.unwrap_or(0);
        update_best_reaction(progress, ms);
        show_hud(hud, format!("Nice! Reaction: {ms} ms"), 2.0);
    } else {
        show_hud(hud, "Reaction failed".to_string(), 1.5);
    }
}

fn update_best_reaction(progress: &mut FunProgress, reaction_ms: u32) {
    progress.best_reaction_ms = Some(
        progress
            .best_reaction_ms
            .map_or(reaction_ms, |best| best.min(reaction_ms)),
    );
}

fn update_achievements(progress: &mut FunProgress, hud: &mut FunHudMessage) {
    try_unlock(
        progress,
        hud,
        Achievement::FirstFeed,
        has_first_feed(progress),
    );
    try_unlock(progress, hud, Achievement::PetLover, is_pet_lover(progress));
    try_unlock(
        progress,
        hud,
        Achievement::ComboStarter,
        has_started_combo(progress),
    );
    try_unlock(
        progress,
        hud,
        Achievement::ReflexAce,
        is_reflex_ace(progress),
    );
}

fn try_unlock(
    progress: &mut FunProgress,
    hud: &mut FunHudMessage,
    achievement: Achievement,
    condition: bool,
) {
    if condition && progress.unlocked.insert(achievement) {
        show_hud(hud, achievement.title(), 2.6);
    }
}

fn show_hud(hud: &mut FunHudMessage, message: impl Into<String>, duration_secs: f32) {
    hud.text = message.into();
    hud.visible = true;
    hud.timer = Timer::from_seconds(duration_secs, TimerMode::Once);
}

fn reaction_start_delay(time: &Time) -> f32 {
    0.8 + (time.elapsed_secs() * 3.7).sin().abs() * 1.2
}

fn pressed_space(keys: &ButtonInput<KeyCode>) -> bool {
    keys.just_pressed(KeyCode::Space)
}

fn finalize_reaction_ms(elapsed_ms: f32) -> u32 {
    elapsed_ms.round().max(1.0) as u32
}

fn emit_reaction_success(fun_events: &mut MessageWriter<FunEvent>, reaction_ms: u32) {
    fun_events.write(FunEvent::ReactionFinished {
        success: true,
        reaction_ms: Some(reaction_ms),
    });
}

fn emit_reaction_failure(fun_events: &mut MessageWriter<FunEvent>, message: &str) {
    fun_events.write(FunEvent::ReactionFinished {
        success: false,
        reaction_ms: None,
    });
    fun_events.write(FunEvent::Notify(message.to_string()));
}

fn go_state() -> ReactionState {
    ReactionState::Go {
        elapsed_ms: 0.0,
        timeout: Timer::from_seconds(1.5, TimerMode::Once),
    }
}

fn cooldown_state(seconds: f32) -> ReactionState {
    ReactionState::Cooldown {
        timer: Timer::from_seconds(seconds, TimerMode::Once),
    }
}

fn has_first_feed(progress: &FunProgress) -> bool {
    progress.feed_count >= 1
}

fn is_pet_lover(progress: &FunProgress) -> bool {
    progress.pet_count >= 10
}

fn has_started_combo(progress: &FunProgress) -> bool {
    progress.combo_count >= 1
}

fn is_reflex_ace(progress: &FunProgress) -> bool {
    progress.best_reaction_ms.is_some_and(|ms| ms <= 350)
}
