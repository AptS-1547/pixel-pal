use bevy::prelude::*;

use crate::animation::UserAction;

#[derive(Message, Debug, Clone)]
pub enum FunEvent {
    ActionTriggered(UserAction),
    ComboTriggered,
    ReactionFinished {
        success: bool,
        reaction_ms: Option<u32>,
    },
    Notify(String),
}
