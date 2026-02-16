use bevy::prelude::*;

use crate::animation::{
    ActionState, ActionStateMachine, AnimationIndices, AnimationTimer, UserAction,
};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index < indices.first || atlas.index >= indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub fn update_action_animation(
    mut query: Query<&mut AnimationIndices>,
    state_machine: Res<ActionStateMachine>,
) {
    for mut indices in &mut query {
        let (first, last) = match state_machine.state {
            ActionState::Idle => (0, 3), // 默认待机
            ActionState::Acting(UserAction::Dance) => (4, 7),
            ActionState::Acting(UserAction::Sleep) => (8, 11),
            ActionState::Acting(UserAction::Talk) => (12, 15),
            ActionState::Acting(UserAction::Feed) => (16, 19),
            ActionState::Acting(UserAction::Pet) => (20, 23),
        };
        indices.first = first;
        indices.last = last;
    }
}

pub fn tick_action_state_machine(
    time: Res<Time>,
    mut state_machine: ResMut<ActionStateMachine>,
    mut pet_query: Query<&mut crate::pet::Pet>,
) {
    let Some(action) = state_machine.current_action() else {
        return;
    };

    state_machine.timer.tick(time.delta());

    for mut pet in &mut pet_query {
        match action {
            UserAction::Dance => {
                pet.happiness = (pet.happiness + 15.0 * time.delta_secs()).min(100.0);
                pet.energy = (pet.energy - 5.0 * time.delta_secs()).max(0.0);
            }
            UserAction::Sleep => {
                pet.energy = (pet.energy + 25.0 * time.delta_secs()).min(100.0);
            }
            UserAction::Talk => {
                pet.happiness = (pet.happiness + 10.0 * time.delta_secs()).min(100.0);
                pet.energy = (pet.energy - 3.0 * time.delta_secs()).max(0.0);
            }
            UserAction::Feed => {
                pet.hunger = (pet.hunger + 30.0 * time.delta_secs()).min(100.0);
            }
            UserAction::Pet => {
                pet.happiness = (pet.happiness + 8.0 * time.delta_secs()).min(100.0);
            }
        }
    }

    if state_machine.timer.just_finished() {
        state_machine.state = ActionState::Idle;
        state_machine.timer.reset();
    }
}
