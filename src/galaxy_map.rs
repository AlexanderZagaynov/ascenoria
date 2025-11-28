//! Galaxy map screen implementation inspired by classic Ascendancy.
//!
//! Displays a star map with clickable star systems and a right-side control panel.

use bevy::{ecs::hierarchy::ChildSpawnerCommands, prelude::*};
use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::galaxy::Galaxy;
use crate::main_menu::GameState;

/// Plugin that manages the galaxy map screen.
pub struct GalaxyMapPlugin;

impl Plugin for GalaxyMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GalaxyMapState>()
            .add_systems(OnEnter(GameState::InGame), setup_galaxy_map)
            .add_systems(OnExit(GameState::InGame), cleanup_galaxy_map)
            .add_systems(
                Update,
                (
                    star_hover_system,
                    panel_button_system,
                    camera_pan_system,
                    turn_control_system,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

/// Marker component for all galaxy map UI entities.
#[derive(Component)]
pub struct GalaxyMapRoot;

/// Marker for star system entities on the map.
#[derive(Component)]
pub struct StarMarker {
    pub system_index: usize,
}

/// Marker for the currently selected star.
#[derive(Component)]
pub struct SelectedStar;

/// State for the galaxy map view.
#[derive(Resource, Default)]
pub struct GalaxyMapState {
    pub selected_system: Option<usize>,
    pub camera_offset: Vec2,
    pub turn_number: u32,
}

/// Side panel button types.
#[derive(Component, Clone, Copy, Debug)]
pub enum PanelButton {
    Planets,
    Ships,
    Research,
    SpecialAbility,
    Species,
    EndTurn,
    NextTurn,
}

/// Colors for the galaxy map UI.
pub mod colors {
    use bevy::prelude::*;

    /// Black space background.
    pub const SPACE_BLACK: Color = Color::srgb(0.0, 0.0, 0.0);
    /// Gray textured panel background.
    pub const PANEL_BG: Color = Color::srgb(0.35, 0.38, 0.42);
    /// Darker panel sections.
    pub const PANEL_DARK: Color = Color::srgb(0.25, 0.28, 0.32);
    /// Panel border color.
    pub const PANEL_BORDER: Color = Color::srgb(0.2, 0.22, 0.25);
    /// Bright green for player-owned systems.
    pub const STAR_PLAYER: Color = Color::srgb(0.2, 0.9, 0.3);
    /// Orange for enemy systems.
    pub const STAR_ENEMY: Color = Color::srgb(0.9, 0.4, 0.1);
    /// White/yellow for neutral stars.
    pub const STAR_NEUTRAL: Color = Color::srgb(0.95, 0.9, 0.7);
    /// Red giant stars.
    pub const STAR_RED: Color = Color::srgb(0.9, 0.3, 0.2);
    /// Blue stars.
    pub const STAR_BLUE: Color = Color::srgb(0.4, 0.6, 0.95);
    /// Cyan for selection highlight.
    pub const SELECTION_CYAN: Color = Color::srgb(0.2, 0.8, 0.8);
    /// Green ring indicators.
    pub const RING_GREEN: Color = Color::srgb(0.3, 0.7, 0.4);
    /// Text on panels.
    pub const PANEL_TEXT: Color = Color::srgb(0.85, 0.85, 0.85);
    /// Dim text.
    pub const PANEL_TEXT_DIM: Color = Color::srgb(0.6, 0.6, 0.6);
}

/// Star types for visual variety.
#[derive(Clone, Copy, Debug)]
pub enum StarType {
    Yellow,
    Orange,
    Red,
    Blue,
    White,
}

impl StarType {
    fn color(&self) -> Color {
        match self {
            StarType::Yellow => Color::srgb(1.0, 0.95, 0.6),
            StarType::Orange => Color::srgb(1.0, 0.6, 0.3),
            StarType::Red => colors::STAR_RED,
            StarType::Blue => colors::STAR_BLUE,
            StarType::White => Color::srgb(0.95, 0.95, 1.0),
        }
    }

    fn from_seed(seed: u64) -> Self {
        match seed % 5 {
            0 => StarType::Yellow,
            1 => StarType::Orange,
            2 => StarType::Red,
            3 => StarType::Blue,
            _ => StarType::White,
        }
    }
}

/// Generate star positions from galaxy data.
fn generate_star_positions(galaxy: &Galaxy, seed: u64) -> Vec<(Vec2, StarType)> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut positions = Vec::with_capacity(galaxy.systems.len());

    // Generate positions in a scattered pattern
    let map_size = 500.0;

    for (i, _system) in galaxy.systems.iter().enumerate() {
        let angle = Rng::r#gen::<f32>(&mut rng) * std::f32::consts::TAU;
        let distance = Rng::r#gen::<f32>(&mut rng).sqrt() * map_size;
        let x = angle.cos() * distance;
        let y = angle.sin() * distance;

        let star_type = StarType::from_seed(seed.wrapping_add(i as u64));
        positions.push((Vec2::new(x, y), star_type));
    }

    positions
}

fn setup_galaxy_map(mut commands: Commands, galaxy_preview: Res<crate::GalaxyPreview>) {
    // Camera for the galaxy view
    commands.spawn((Camera2d::default(), GalaxyMapRoot));

    // Generate star positions
    let star_positions = generate_star_positions(&galaxy_preview.galaxy, 1337);

    // Spawn star markers as sprites
    for (i, (pos, star_type)) in star_positions.iter().enumerate() {
        // Main star sprite
        commands.spawn((
            Sprite {
                color: star_type.color(),
                custom_size: Some(Vec2::splat(8.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            StarMarker { system_index: i },
            GalaxyMapRoot,
        ));

        // Add a glow effect (larger, dimmer sprite behind)
        commands.spawn((
            Sprite {
                color: star_type.color().with_alpha(0.3),
                custom_size: Some(Vec2::splat(16.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(pos.x, pos.y, -0.1)),
            GalaxyMapRoot,
        ));
    }

    // Add some background stars (small dots)
    let mut bg_rng = StdRng::seed_from_u64(42);
    for _ in 0..150 {
        let x = Rng::gen_range(&mut bg_rng, -600.0..600.0);
        let y = Rng::gen_range(&mut bg_rng, -400.0..400.0);
        let brightness = Rng::gen_range(&mut bg_rng, 0.2..0.6);
        let size = Rng::gen_range(&mut bg_rng, 1.0..3.0);

        commands.spawn((
            Sprite {
                color: Color::srgba(brightness, brightness, brightness * 1.1, 0.8),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, -1.0)),
            GalaxyMapRoot,
        ));
    }

    // UI overlay - right side panel
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            GalaxyMapRoot,
        ))
        .with_children(|parent| {
            // Right panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(colors::PANEL_BG),
                ))
                .with_children(|panel| {
                    // Turn indicators at top
                    spawn_turn_indicators(panel);

                    // Speed controls
                    spawn_speed_controls(panel);

                    // Main menu buttons
                    spawn_panel_section(panel, "Planets", PanelButton::Planets);
                    spawn_panel_section(panel, "Ships", PanelButton::Ships);
                    spawn_panel_section(panel, "Research", PanelButton::Research);
                    spawn_panel_section(panel, "Special Ability", PanelButton::SpecialAbility);
                    spawn_panel_section(panel, "Species", PanelButton::Species);

                    // Spacer
                    panel.spawn(Node {
                        flex_grow: 1.0,
                        ..default()
                    });

                    // Bottom control buttons
                    spawn_bottom_controls(panel);
                });
        });

    // Player icon in top-left (species indicator)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::RING_GREEN),
            GalaxyMapRoot,
        ))
        .with_children(|icon| {
            icon.spawn((
                Text::new("⬡"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::RING_GREEN),
            ));
        });

    // Instructions
    commands.spawn((
        Text::new("Click stars to select • Drag to pan • ESC for menu"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(colors::PANEL_TEXT_DIM),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        GalaxyMapRoot,
    ));
}

fn spawn_turn_indicators(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        })
        .with_children(|row| {
            // 5 ring indicators (like in Ascendancy)
            for _ in 0..5 {
                row.spawn((
                    Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    BorderColor::all(colors::RING_GREEN),
                    BorderRadius::all(Val::Percent(50.0)),
                ));
            }
        });
}

fn spawn_speed_controls(panel: &mut ChildSpawnerCommands) {
    panel
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                margin: UiRect::bottom(Val::Px(12.0)),
                column_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
        ))
        .with_children(|row| {
            // Clock + arrow speed indicators
            for label in ["⏱→", "⏱⇒"] {
                row.spawn((
                    Node {
                        padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(colors::PANEL_DARK),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::PANEL_TEXT),
                    ));
                });
            }
        });
}

fn spawn_panel_section(panel: &mut ChildSpawnerCommands, label: &str, button_type: PanelButton) {
    panel
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                margin: UiRect::bottom(Val::Px(4.0)),
                padding: UiRect::horizontal(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::PANEL_BORDER),
            button_type,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::PANEL_TEXT),
            ));

            // Icon placeholder
            btn.spawn((
                Node {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.3, 0.4, 0.8)),
                BorderColor::all(colors::PANEL_BORDER),
            ))
            .with_children(|icon| {
                let icon_char = match button_type {
                    PanelButton::Planets => "🌍",
                    PanelButton::Ships => "🚀",
                    PanelButton::Research => "🔬",
                    PanelButton::SpecialAbility => "✨",
                    PanelButton::Species => "👽",
                    _ => "•",
                };
                icon.spawn((
                    Text::new(icon_char),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(colors::PANEL_TEXT),
                ));
            });
        });
}

fn spawn_bottom_controls(panel: &mut ChildSpawnerCommands) {
    // Grid of circular control buttons (like in Ascendancy)
    panel
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            margin: UiRect::top(Val::Px(8.0)),
            ..default()
        })
        .with_children(|grid| {
            // First row of 4 buttons
            grid.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            })
            .with_children(|row| {
                for (icon, color) in [
                    ("−", Color::srgb(0.5, 0.6, 0.7)),
                    ("!", Color::srgb(0.8, 0.3, 0.2)),
                    ("▲", Color::srgb(0.8, 0.5, 0.3)),
                    ("+", Color::srgb(0.4, 0.5, 0.6)),
                ] {
                    spawn_circular_button(row, icon, color);
                }
            });

            // Second row of 4 buttons
            grid.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            })
            .with_children(|row| {
                for (icon, color) in [
                    ("⚙", Color::srgb(0.6, 0.5, 0.3)),
                    ("☆", Color::srgb(0.7, 0.6, 0.2)),
                    ("◎", Color::srgb(0.5, 0.3, 0.5)),
                    ("◉", Color::srgb(0.3, 0.4, 0.6)),
                ] {
                    spawn_circular_button(row, icon, color);
                }
            });

            // Bottom row - speed indicators
            grid.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            })
            .with_children(|row| {
                for i in 0..5 {
                    row.spawn((
                        Node {
                            width: Val::Px(36.0),
                            height: Val::Px(24.0),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(colors::PANEL_DARK),
                        BorderColor::all(colors::PANEL_BORDER),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(format!("{}", i + 1)),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(colors::PANEL_TEXT_DIM),
                        ));
                    });
                }
            });
        });
}

fn spawn_circular_button(parent: &mut ChildSpawnerCommands, icon: &str, bg_color: Color) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(44.0),
                height: Val::Px(44.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color.with_alpha(0.8)),
            BorderColor::all(colors::PANEL_BORDER),
            BorderRadius::all(Val::Percent(50.0)),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(icon),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::PANEL_TEXT),
            ));
        });
}

fn cleanup_galaxy_map(mut commands: Commands, query: Query<Entity, With<GalaxyMapRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handle hovering over stars.
fn star_hover_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<GalaxyMapRoot>>,
    mut star_query: Query<(&StarMarker, &Transform, &mut Sprite)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut map_state: ResMut<GalaxyMapState>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let mut hovered_star: Option<usize> = None;

    for (marker, transform, mut sprite) in &mut star_query {
        let distance = world_position.distance(transform.translation.truncate());

        if distance < 15.0 {
            hovered_star = Some(marker.system_index);
            sprite.custom_size = Some(Vec2::splat(12.0)); // Enlarge on hover
        } else {
            sprite.custom_size = Some(Vec2::splat(8.0)); // Normal size
        }
    }

    // Handle click to select
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(idx) = hovered_star {
            map_state.selected_system = Some(idx);
            info!("Selected system {}", idx);
        }
    }
}

/// Handle panel button interactions.
fn panel_button_system(
    mut interaction_query: Query<
        (&Interaction, &PanelButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Panel button pressed: {:?}", button);
                *bg_color = BackgroundColor(colors::PANEL_DARK.with_alpha(1.0));
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.35, 0.38, 0.42));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(colors::PANEL_DARK);
            }
        }
    }
}

/// Handle camera panning with right mouse drag.
fn camera_pan_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_events: bevy::ecs::message::MessageReader<bevy::input::mouse::MouseMotion>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, With<GalaxyMapRoot>)>,
) {
    if !buttons.pressed(MouseButton::Right) {
        motion_events.clear();
        return;
    }

    let mut delta = Vec2::ZERO;
    for event in motion_events.read() {
        delta += event.delta;
    }

    if delta != Vec2::ZERO {
        for mut transform in &mut camera_query {
            transform.translation.x -= delta.x;
            transform.translation.y += delta.y;
        }
    }
}

/// Handle turn controls.
fn turn_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut map_state: ResMut<GalaxyMapState>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        map_state.turn_number += 1;
        info!("Turn {}", map_state.turn_number);
    }
}
