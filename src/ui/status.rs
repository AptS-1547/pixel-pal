use bevy::prelude::*;

use super::components::{
    FunToastPanel, FunToastText, StatusValueEnergy, StatusValueHeart, StatusValueHunger,
};

pub fn update_status_display(
    pet_query: Query<&crate::pet::Pet>,
    mut param_set: ParamSet<(
        StatusValueHeartQuery,
        StatusValueHungerQuery,
        StatusValueEnergyQuery,
    )>,
) {
    for pet in &pet_query {
        let happiness = pet.happiness.clamp(0.0, 100.0).round() as i32;
        let hunger = pet.hunger.clamp(0.0, 100.0).round() as i32;
        let energy = pet.energy.clamp(0.0, 100.0).round() as i32;

        for (mut text, mut color) in &mut param_set.p0() {
            text.0 = happiness.to_string();
            color.0 = status_text_color(pet.happiness);
        }
        for (mut text, mut color) in &mut param_set.p1() {
            text.0 = hunger.to_string();
            color.0 = status_text_color(pet.hunger);
        }
        for (mut text, mut color) in &mut param_set.p2() {
            text.0 = energy.to_string();
            color.0 = status_text_color(pet.energy);
        }
    }
}

pub fn update_fun_toast_display(
    hud_message: Res<crate::fun::FunHudMessage>,
    mut panel_query: Query<&mut Visibility, With<FunToastPanel>>,
    mut text_query: Query<&mut Text, With<FunToastText>>,
) {
    if !hud_message.is_changed() {
        return;
    }

    for mut visibility in &mut panel_query {
        *visibility = if hud_message.visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    for mut text in &mut text_query {
        text.0 = hud_message.text.clone();
    }
}

fn status_text_color(value: f32) -> Color {
    if value >= 60.0 {
        Color::srgb(0.56, 0.95, 0.76)
    } else if value >= 30.0 {
        Color::srgb(0.98, 0.83, 0.39)
    } else {
        Color::srgb(0.97, 0.45, 0.45)
    }
}

type StatusValueHeartQuery<'w, 's> =
    Query<'w, 's, (&'static mut Text, &'static mut TextColor), With<StatusValueHeart>>;
type StatusValueHungerQuery<'w, 's> =
    Query<'w, 's, (&'static mut Text, &'static mut TextColor), With<StatusValueHunger>>;
type StatusValueEnergyQuery<'w, 's> =
    Query<'w, 's, (&'static mut Text, &'static mut TextColor), With<StatusValueEnergy>>;
