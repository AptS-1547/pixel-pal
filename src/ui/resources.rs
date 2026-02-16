use bevy::prelude::*;

#[derive(Resource)]
#[allow(dead_code)]
pub struct Icons {
    pub dance: Handle<Image>,
    pub sleep: Handle<Image>,
    pub talk: Handle<Image>,
    pub feed: Handle<Image>,
    pub pet: Handle<Image>,
    pub heart: Handle<Image>,
    pub hunger: Handle<Image>,
    pub energy: Handle<Image>,
}

// 菜单状态
#[derive(Resource, Default, PartialEq, Eq)]
pub enum MenuState {
    #[default]
    Hidden,
    Visible,
}
