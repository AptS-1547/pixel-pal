use bevy::prelude::*;

mod animation;
mod fun;
mod pet;
mod ui;
mod window;

use animation::{
    ActionStateMachine, animate_sprite, tick_action_state_machine, update_action_animation,
};
use fun::{
    ComboTracker, FunEvent, FunHudMessage, FunProgress, ReactionGame, process_fun_events,
    tick_fun_hud_message, update_reaction_game,
};
use ui::{
    handle_click_outside, handle_menu_actions, handle_menu_hotkey, handle_pet_click, setup_ui,
    sync_menu_visibility, update_fun_toast_display, update_status_display,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(ActionStateMachine::default())
        .insert_resource(ComboTracker::default())
        .insert_resource(ReactionGame::default())
        .insert_resource(FunProgress::default())
        .insert_resource(FunHudMessage::default())
        .add_message::<FunEvent>()
        .add_plugins(DefaultPlugins.set(window::window_plugin()))
        .add_systems(Startup, setup_ui)
        .add_systems(
            Update,
            (
                pet::update_pet_state,
                update_reaction_game,
                handle_pet_click,
                handle_click_outside,
                handle_menu_hotkey,
                sync_menu_visibility,
                handle_menu_actions,
                process_fun_events,
                tick_fun_hud_message,
                tick_action_state_machine,
                update_action_animation,
                animate_sprite,
                update_status_display,
                update_fun_toast_display,
            )
                .chain(),
        )
        .run();
}
