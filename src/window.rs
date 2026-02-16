use bevy::prelude::*;
use bevy::window::{
    CompositeAlphaMode, CursorOptions, WindowLevel, WindowMode, WindowPlugin, WindowPosition,
};

pub fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(bevy::window::Window {
            title: "PixelPal".into(),
            resolution: (140_u32, 180_u32).into(),
            decorations: false,
            transparent: true,
            resizable: false,
            movable_by_window_background: true,
            mode: WindowMode::Windowed,
            window_level: WindowLevel::AlwaysOnTop,
            position: WindowPosition::Automatic,
            #[cfg(target_os = "macos")]
            composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
            #[cfg(target_os = "linux")]
            composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
            ..default()
        }),
        primary_cursor_options: Some(CursorOptions {
            hit_test: true,
            ..default()
        }),
        ..default()
    }
}
