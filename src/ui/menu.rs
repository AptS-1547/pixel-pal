use bevy::prelude::*;

use crate::animation::{ActionStateMachine, UserAction};
use crate::fun::{ComboTracker, FunEvent, ReactionGame};

use super::components::{
    ActionMenuContainer, DanceButton, FeedButton, PetButton, SleepButton, TalkButton,
};
use super::resources::MenuState;

// 点击机器人切换菜单
#[allow(clippy::too_many_arguments)]
pub fn handle_pet_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
    mut menu_state: ResMut<MenuState>,
    mut action_state_machine: ResMut<ActionStateMachine>,
    reaction_game: Res<ReactionGame>,
    mut combo_tracker: ResMut<ComboTracker>,
    mut fun_events: MessageWriter<FunEvent>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    if !action_state_machine.is_idle() || reaction_game.is_active() {
        return;
    }

    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    let half_size = 64.0;
    if world_pos.x >= -half_size
        && world_pos.x <= half_size
        && world_pos.y >= -half_size
        && world_pos.y <= half_size
    {
        if combo_tracker.register_click(time.elapsed_secs_f64()) {
            action_state_machine.trigger(UserAction::Dance);
            *menu_state = MenuState::Hidden;
            fun_events.write(FunEvent::ComboTriggered);
            return;
        }

        *menu_state = if *menu_state == MenuState::Visible {
            MenuState::Hidden
        } else {
            MenuState::Visible
        };
    }
}

// 同步菜单可见性
pub fn sync_menu_visibility(
    mut menu: Query<&mut Visibility, With<ActionMenuContainer>>,
    menu_state: Res<MenuState>,
) {
    if menu_state.is_changed() {
        let target = if *menu_state == MenuState::Visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        for mut visibility in &mut menu {
            *visibility = target;
        }
    }
}

pub fn handle_menu_hotkey(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
    action_state_machine: Res<ActionStateMachine>,
    reaction_game: Res<ReactionGame>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    if !action_state_machine.is_idle() || reaction_game.is_active() {
        return;
    }

    *menu_state = if *menu_state == MenuState::Visible {
        MenuState::Hidden
    } else {
        MenuState::Visible
    };
}

// 点击空白区域关闭菜单
pub fn handle_click_outside(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    menu_buttons: MenuButtonsQuery,
    mut menu_state: ResMut<MenuState>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    if *menu_state == MenuState::Hidden {
        return;
    }

    for interaction in &menu_buttons {
        if *interaction == Interaction::Pressed {
            return;
        }
    }

    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    let half_size = 64.0;
    if world_pos.x >= -half_size
        && world_pos.x <= half_size
        && world_pos.y >= -half_size
        && world_pos.y <= half_size
    {
        return;
    }

    *menu_state = MenuState::Hidden;
}

// 处理菜单按钮点击
#[allow(clippy::too_many_arguments)]
pub fn handle_menu_actions(
    dance_query: Query<&Interaction, With<DanceButton>>,
    sleep_query: Query<&Interaction, With<SleepButton>>,
    talk_query: Query<&Interaction, With<TalkButton>>,
    feed_query: Query<&Interaction, With<FeedButton>>,
    pet_query: Query<&Interaction, With<PetButton>>,
    mut action_state_machine: ResMut<ActionStateMachine>,
    mut menu_state: ResMut<MenuState>,
    mut fun_events: MessageWriter<FunEvent>,
) {
    let new_action = if dance_query.iter().any(|i| *i == Interaction::Pressed) {
        Some(UserAction::Dance)
    } else if sleep_query.iter().any(|i| *i == Interaction::Pressed) {
        Some(UserAction::Sleep)
    } else if talk_query.iter().any(|i| *i == Interaction::Pressed) {
        Some(UserAction::Talk)
    } else if feed_query.iter().any(|i| *i == Interaction::Pressed) {
        Some(UserAction::Feed)
    } else if pet_query.iter().any(|i| *i == Interaction::Pressed) {
        Some(UserAction::Pet)
    } else {
        None
    };

    if let Some(a) = new_action {
        action_state_machine.trigger(a);
        fun_events.write(FunEvent::ActionTriggered(a));
        *menu_state = MenuState::Hidden;
    }
}

type MenuButtonsQuery<'w, 's> = Query<
    'w,
    's,
    &'static Interaction,
    Or<(
        With<DanceButton>,
        With<SleepButton>,
        With<TalkButton>,
        With<FeedButton>,
        With<PetButton>,
    )>,
>;
