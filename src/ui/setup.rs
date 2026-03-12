use bevy::prelude::*;

use crate::animation::{AnimationIndices, AnimationTimer};

use super::components::{
    ActionMenuContainer, DanceButton, FeedButton, FunToastPanel, FunToastText, PetButton,
    PetSprite, SleepButton, StatusDisplay, StatusValueEnergy, StatusValueHeart, StatusValueHunger,
    TalkButton,
};
use super::resources::{Icons, MenuState};

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_camera(&mut commands);
    init_ui_resources(&mut commands, &asset_server);
    spawn_pet_sprite(&mut commands, &asset_server, &mut textures);
    spawn_status_panel(&mut commands, &asset_server);
    spawn_fun_toast(&mut commands);
    spawn_action_menu(&mut commands, &asset_server);
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2d);
}

fn init_ui_resources(commands: &mut Commands, asset_server: &AssetServer) {
    commands.insert_resource(MenuState::Hidden);
    commands.insert_resource(Icons {
        dance: asset_server.load("icons/dance.png"),
        sleep: asset_server.load("icons/sleep.png"),
        talk: asset_server.load("icons/talk.png"),
        feed: asset_server.load("icons/feed.png"),
        pet: asset_server.load("icons/pet.png"),
        heart: asset_server.load("icons/heart.png"),
        hunger: asset_server.load("icons/hunger.png"),
        energy: asset_server.load("icons/energy.png"),
    });
}

fn spawn_pet_sprite(
    commands: &mut Commands,
    asset_server: &AssetServer,
    textures: &mut Assets<TextureAtlasLayout>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(204, 218), 4, 6, None, None);
    let texture_atlas_layout = textures.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        Sprite::from_atlas_image(
            asset_server.load("textures/robo_1547.png"),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.6)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        crate::pet::Pet::new(),
        PetSprite,
    ));
}

fn spawn_status_panel(commands: &mut Commands, asset_server: &AssetServer) {
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(122.0),
                        height: Val::Px(30.0),
                        top: Val::Px(-4.0),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        padding: UiRect::axes(Val::Px(6.0), Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.06, 0.1, 0.18, 0.86)),
                    StatusDisplay,
                ))
                .with_children(|status_row| {
                    spawn_status_metric(
                        status_row,
                        asset_server,
                        "icons/heart.png",
                        StatusMetric::Heart,
                    );
                    spawn_status_metric(
                        status_row,
                        asset_server,
                        "icons/hunger.png",
                        StatusMetric::Hunger,
                    );
                    spawn_status_metric(
                        status_row,
                        asset_server,
                        "icons/energy.png",
                        StatusMetric::Energy,
                    );
                });
        });
}

fn spawn_fun_toast(commands: &mut Commands) {
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(8.0),
                        width: Val::Px(132.0),
                        height: Val::Px(20.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.05, 0.08, 0.15, 0.9)),
                    FunToastPanel,
                    Visibility::Hidden,
                ))
                .with_children(|box_node| {
                    box_node.spawn((
                        Text::new(""),
                        TextFont {
                            font_size: 8.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.97, 1.0)),
                        FunToastText,
                    ));
                });
        });
}

fn spawn_status_metric(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    icon_path: &'static str,
    metric: StatusMetric,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(34.0),
                height: Val::Px(18.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.13, 0.18, 0.3, 0.9)),
        ))
        .with_children(|cell| {
            cell.spawn((
                ImageNode::new(asset_server.load(icon_path)),
                Node {
                    width: Val::Px(10.0),
                    height: Val::Px(10.0),
                    ..default()
                },
            ));

            match metric {
                StatusMetric::Heart => {
                    cell.spawn((
                        Text::new("100"),
                        TextFont {
                            font_size: 8.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        StatusValueHeart,
                    ));
                }
                StatusMetric::Hunger => {
                    cell.spawn((
                        Text::new("100"),
                        TextFont {
                            font_size: 8.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        StatusValueHunger,
                    ));
                }
                StatusMetric::Energy => {
                    cell.spawn((
                        Text::new("100"),
                        TextFont {
                            font_size: 8.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        StatusValueEnergy,
                    ));
                }
            }
        });
}

fn spawn_action_menu(commands: &mut Commands, asset_server: &AssetServer) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .with_children(|root| {
            root.spawn(Node {
                position_type: PositionType::Absolute,
                right: Val::Px(80.0),
                width: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(6.0)),
                row_gap: Val::Px(4.0),
                ..default()
            })
            .insert(BackgroundColor(Color::srgba(0.1, 0.12, 0.2, 0.95)))
            .insert(ActionMenuContainer)
            .insert(Visibility::Hidden)
            .with_children(|menu| {
                spawn_action_button(
                    menu,
                    asset_server,
                    ActionButtonSpec::new(
                        "Dance",
                        "icons/dance.png",
                        Color::srgb(0.9, 0.3, 0.5),
                        ActionButtonKind::Dance,
                    ),
                );
                spawn_action_button(
                    menu,
                    asset_server,
                    ActionButtonSpec::new(
                        "Sleep",
                        "icons/sleep.png",
                        Color::srgb(0.3, 0.4, 0.7),
                        ActionButtonKind::Sleep,
                    ),
                );
                spawn_action_button(
                    menu,
                    asset_server,
                    ActionButtonSpec::new(
                        "Talk",
                        "icons/talk.png",
                        Color::srgb(0.3, 0.6, 0.4),
                        ActionButtonKind::Talk,
                    ),
                );
                spawn_action_button(
                    menu,
                    asset_server,
                    ActionButtonSpec::new(
                        "Feed",
                        "icons/feed.png",
                        Color::srgb(0.7, 0.5, 0.2),
                        ActionButtonKind::Feed,
                    ),
                );
                spawn_action_button(
                    menu,
                    asset_server,
                    ActionButtonSpec::new(
                        "Pet",
                        "icons/pet.png",
                        Color::srgb(0.6, 0.3, 0.6),
                        ActionButtonKind::Pet,
                    ),
                );
            });
        });
}

fn spawn_action_button(
    menu: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    spec: ActionButtonSpec,
) {
    let mut button = menu.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(26.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            padding: UiRect::left(Val::Px(4.0)),
            column_gap: Val::Px(6.0),
            ..default()
        },
        BackgroundColor(spec.color),
        Button,
    ));

    match spec.kind {
        ActionButtonKind::Dance => {
            button.insert(DanceButton);
        }
        ActionButtonKind::Sleep => {
            button.insert(SleepButton);
        }
        ActionButtonKind::Talk => {
            button.insert(TalkButton);
        }
        ActionButtonKind::Feed => {
            button.insert(FeedButton);
        }
        ActionButtonKind::Pet => {
            button.insert(PetButton);
        }
    }

    button.with_children(|btn| {
        btn.spawn((
            ImageNode::new(asset_server.load(spec.icon_path)),
            Node {
                width: Val::Px(18.0),
                height: Val::Px(18.0),
                ..default()
            },
        ));
        btn.spawn((
            Text::new(spec.label),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

#[derive(Clone, Copy)]
enum StatusMetric {
    Heart,
    Hunger,
    Energy,
}

#[derive(Clone, Copy)]
enum ActionButtonKind {
    Dance,
    Sleep,
    Talk,
    Feed,
    Pet,
}

#[derive(Clone, Copy)]
struct ActionButtonSpec {
    label: &'static str,
    icon_path: &'static str,
    color: Color,
    kind: ActionButtonKind,
}

impl ActionButtonSpec {
    fn new(
        label: &'static str,
        icon_path: &'static str,
        color: Color,
        kind: ActionButtonKind,
    ) -> Self {
        Self {
            label,
            icon_path,
            color,
            kind,
        }
    }
}
