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
            .init_resource::<InfoModalState>()
            .add_systems(OnEnter(GameState::InGame), setup_galaxy_map)
            .add_systems(OnExit(GameState::InGame), cleanup_galaxy_map)
            .add_systems(
                Update,
                (
                    star_hover_system,
                    panel_button_system,
                    camera_pan_system,
                    turn_control_system,
                    info_modal_system,
                    info_modal_button_system,
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

// ============================================================================
// Info Modal System
// ============================================================================

/// Types of icons that can be displayed in the info modal.
#[derive(Clone, Debug, Default)]
pub enum ModalIcon {
    /// Factory/industry building icon.
    Factory,
    /// Research/lab building icon.
    Laboratory,
    /// Shipyard/construction icon.
    Shipyard,
    /// Defense/military icon.
    Defense,
    /// Planet/colony icon.
    Planet,
    /// Ship icon.
    Ship,
    /// Research breakthrough icon.
    Research,
    /// Warning/alert icon.
    Warning,
    /// No icon.
    #[default]
    None,
}

/// Action for modal buttons.
#[derive(Clone, Debug)]
pub enum ModalAction {
    /// Close the modal.
    Dismiss,
    /// Navigate to a specific planet.
    GoToPlanet { system_index: usize, planet_index: usize },
    /// Navigate to a specific star system.
    GoToSystem { system_index: usize },
    /// Open research screen.
    OpenResearch,
    /// Open ship designer.
    OpenShipDesign,
}

/// Configuration for a modal button.
#[derive(Clone, Debug)]
pub struct ModalButton {
    pub label: String,
    pub action: ModalAction,
}

/// State resource for the info modal dialog.
#[derive(Resource, Default)]
pub struct InfoModalState {
    /// Whether the modal is currently visible.
    pub visible: bool,
    /// Icon to display (optional).
    pub icon: ModalIcon,
    /// Main message text.
    pub message: String,
    /// Optional secondary/detail text.
    pub detail: Option<String>,
    /// Buttons to display.
    pub buttons: Vec<ModalButton>,
}

impl InfoModalState {
    /// Create a simple notification with just an OK button.
    pub fn notification(icon: ModalIcon, message: impl Into<String>) -> Self {
        Self {
            visible: true,
            icon,
            message: message.into(),
            detail: None,
            buttons: vec![ModalButton {
                label: "OK".to_string(),
                action: ModalAction::Dismiss,
            }],
        }
    }

    /// Create a notification with a "Go to Planet" and "OK" buttons.
    pub fn planet_notification(
        icon: ModalIcon,
        message: impl Into<String>,
        system_index: usize,
        planet_index: usize,
    ) -> Self {
        Self {
            visible: true,
            icon,
            message: message.into(),
            detail: None,
            buttons: vec![
                ModalButton {
                    label: "Go to Planet".to_string(),
                    action: ModalAction::GoToPlanet { system_index, planet_index },
                },
                ModalButton {
                    label: "OK".to_string(),
                    action: ModalAction::Dismiss,
                },
            ],
        }
    }

    /// Create a notification with custom buttons.
    pub fn custom(
        icon: ModalIcon,
        message: impl Into<String>,
        detail: Option<String>,
        buttons: Vec<ModalButton>,
    ) -> Self {
        Self {
            visible: true,
            icon,
            message: message.into(),
            detail,
            buttons,
        }
    }

    /// Hide the modal.
    pub fn hide(&mut self) {
        self.visible = false;
    }
}

/// Marker for the modal overlay.
#[derive(Component)]
pub struct InfoModalOverlay;

/// Marker for modal buttons with their action.
#[derive(Component)]
pub struct InfoModalButton {
    pub action: ModalAction,
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
    mut next_state: ResMut<NextState<GameState>>,
    mut star_system_state: ResMut<crate::star_system::StarSystemState>,
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

    // Handle click to select, double-click to enter system
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(idx) = hovered_star {
            if map_state.selected_system == Some(idx) {
                // Double-click on same star - enter system view
                star_system_state.system_index = idx;
                star_system_state.selected_planet = None;
                next_state.set(GameState::StarSystem);
                info!("Entering system {}", idx);
            } else {
                map_state.selected_system = Some(idx);
                info!("Selected system {}", idx);
            }
        }
    }
}

/// Handle panel button interactions.
fn panel_button_system(
    mut interaction_query: Query<
        (&Interaction, &PanelButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    map_state: Res<GalaxyMapState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut star_system_state: ResMut<crate::star_system::StarSystemState>,
) {
    for (interaction, button, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Panel button pressed: {:?}", button);
                *bg_color = BackgroundColor(colors::PANEL_DARK.with_alpha(1.0));

                // Handle button actions
                match button {
                    PanelButton::Planets => {
                        if let Some(system_idx) = map_state.selected_system {
                            star_system_state.system_index = system_idx;
                            star_system_state.selected_planet = None;
                            next_state.set(GameState::StarSystem);
                            info!("Entering system {} via Planets button", system_idx);
                        }
                    }
                    _ => {}
                }
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
    mut modal_state: ResMut<InfoModalState>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        map_state.turn_number += 1;
        info!("Turn {}", map_state.turn_number);

        // Demo: Show a notification every 5 turns
        if map_state.turn_number % 5 == 0 {
            *modal_state = InfoModalState::planet_notification(
                ModalIcon::Factory,
                format!("Factory construction complete on Terra Prime (Turn {})", map_state.turn_number),
                0,
                0,
            );
        }
    }

    // Press 'N' to show a test notification
    if keyboard.just_pressed(KeyCode::KeyN) {
        *modal_state = InfoModalState::planet_notification(
            ModalIcon::Factory,
            "Factory construction complete on Terra Prime",
            0,
            0,
        );
    }

    // Press 'M' to show a research notification
    if keyboard.just_pressed(KeyCode::KeyM) {
        *modal_state = InfoModalState::custom(
            ModalIcon::Research,
            "Research Complete: Advanced Propulsion",
            Some("Your scientists have discovered improved engine technology.".to_string()),
            vec![
                ModalButton {
                    label: "View Research".to_string(),
                    action: ModalAction::OpenResearch,
                },
                ModalButton {
                    label: "OK".to_string(),
                    action: ModalAction::Dismiss,
                },
            ],
        );
    }
}

// ============================================================================
// Info Modal Implementation
// ============================================================================

/// Spawn the info modal when visible.
fn info_modal_system(
    mut commands: Commands,
    modal_state: Res<InfoModalState>,
    modal_query: Query<Entity, With<InfoModalOverlay>>,
) {
    // Only process if state changed
    if !modal_state.is_changed() {
        return;
    }

    // Despawn existing modal if any
    for entity in modal_query.iter() {
        commands.entity(entity).despawn();
    }

    // If not visible, we're done
    if !modal_state.visible {
        return;
    }

    // Spawn the modal
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            GlobalZIndex(100), // Above all other UI
            InfoModalOverlay,
            GalaxyMapRoot,
        ))
        .with_children(|parent| {
            // Modal panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(420.0),
                        min_height: Val::Px(200.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(0.0)),
                        ..default()
                    },
                    BackgroundColor(colors::PANEL_BG),
                    BorderRadius::all(Val::Px(8.0)),
                ))
                .with_children(|panel| {
                    // Top section with icon and planet preview
                    panel
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(30.0),
                                padding: UiRect::all(Val::Px(15.0)),
                                ..default()
                            },
                            BackgroundColor(colors::PANEL_DARK),
                            BorderRadius::top(Val::Px(8.0)),
                        ))
                        .with_children(|top| {
                            // Icon (building/event type)
                            let icon_text = match &modal_state.icon {
                                ModalIcon::Factory => "🏭",
                                ModalIcon::Laboratory => "🔬",
                                ModalIcon::Shipyard => "🚀",
                                ModalIcon::Defense => "🛡️",
                                ModalIcon::Planet => "🌍",
                                ModalIcon::Ship => "🛸",
                                ModalIcon::Research => "💡",
                                ModalIcon::Warning => "⚠️",
                                ModalIcon::None => "",
                            };

                            if !icon_text.is_empty() {
                                // Icon container (styled like isometric building)
                                top.spawn((
                                    Node {
                                        width: Val::Px(70.0),
                                        height: Val::Px(70.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.2, 0.22, 0.25)),
                                    BorderRadius::all(Val::Px(6.0)),
                                ))
                                .with_children(|icon_box| {
                                    icon_box.spawn((
                                        Text::new(icon_text),
                                        TextFont {
                                            font_size: 36.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });

                                // Planet preview circle (placeholder)
                                top.spawn((
                                    Node {
                                        width: Val::Px(70.0),
                                        height: Val::Px(70.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.3, 0.5, 0.7)),
                                    BorderRadius::all(Val::Px(35.0)),
                                ))
                                .with_children(|planet_preview| {
                                    // Simple planet visual
                                    planet_preview.spawn((
                                        Node {
                                            width: Val::Px(60.0),
                                            height: Val::Px(60.0),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.2, 0.6, 0.3)),
                                        BorderRadius::all(Val::Px(30.0)),
                                    ));
                                });
                            }
                        });

                    // Message section
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::new(
                                Val::Px(25.0),
                                Val::Px(25.0),
                                Val::Px(20.0),
                                Val::Px(10.0),
                            ),
                            ..default()
                        })
                        .with_children(|msg_section| {
                            // Main message
                            msg_section.spawn((
                                Text::new(&modal_state.message),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(colors::PANEL_TEXT),
                                TextLayout::new_with_justify(Justify::Center),
                                Node {
                                    max_width: Val::Px(350.0),
                                    ..default()
                                },
                            ));

                            // Detail text if present
                            if let Some(ref detail) = modal_state.detail {
                                msg_section.spawn((
                                    Text::new(detail),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(colors::PANEL_TEXT_DIM),
                                    TextLayout::new_with_justify(Justify::Center),
                                    Node {
                                        margin: UiRect::top(Val::Px(8.0)),
                                        max_width: Val::Px(350.0),
                                        ..default()
                                    },
                                ));
                            }
                        });

                    // Button row
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(15.0),
                            padding: UiRect::new(
                                Val::Px(25.0),
                                Val::Px(25.0),
                                Val::Px(10.0),
                                Val::Px(20.0),
                            ),
                            ..default()
                        })
                        .with_children(|button_row| {
                            for modal_button in &modal_state.buttons {
                                let is_primary = !matches!(modal_button.action, ModalAction::Dismiss);
                                let bg_color = if is_primary {
                                    Color::srgb(0.2, 0.5, 0.6)
                                } else {
                                    colors::PANEL_DARK
                                };

                                button_row
                                    .spawn((
                                        Button,
                                        Node {
                                            padding: UiRect::new(
                                                Val::Px(20.0),
                                                Val::Px(20.0),
                                                Val::Px(10.0),
                                                Val::Px(10.0),
                                            ),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(bg_color),
                                        BorderRadius::all(Val::Px(4.0)),
                                        InfoModalButton {
                                            action: modal_button.action.clone(),
                                        },
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new(&modal_button.label),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(colors::PANEL_TEXT),
                                        ));
                                    });
                            }
                        });
                });
        });
}

/// Handle modal button clicks.
fn info_modal_button_system(
    mut interaction_query: Query<
        (&Interaction, &InfoModalButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut modal_state: ResMut<InfoModalState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut star_system_state: ResMut<crate::star_system::StarSystemState>,
    mut planet_view_state: ResMut<crate::planet_view::PlanetViewState>,
) {
    for (interaction, modal_button, mut bg_color) in &mut interaction_query {
        let is_primary = !matches!(modal_button.action, ModalAction::Dismiss);
        let base_color = if is_primary {
            Color::srgb(0.2, 0.5, 0.6)
        } else {
            colors::PANEL_DARK
        };

        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(base_color.with_alpha(0.8));

                match &modal_button.action {
                    ModalAction::Dismiss => {
                        modal_state.hide();
                    }
                    ModalAction::GoToPlanet { system_index, planet_index } => {
                        star_system_state.system_index = *system_index;
                        star_system_state.selected_planet = Some(*planet_index);
                        planet_view_state.planet_index = *planet_index;
                        modal_state.hide();
                        next_state.set(GameState::PlanetView);
                    }
                    ModalAction::GoToSystem { system_index } => {
                        star_system_state.system_index = *system_index;
                        star_system_state.selected_planet = None;
                        modal_state.hide();
                        next_state.set(GameState::StarSystem);
                    }
                    ModalAction::OpenResearch => {
                        modal_state.hide();
                        // TODO: Navigate to research screen
                        info!("Open research (not yet implemented)");
                    }
                    ModalAction::OpenShipDesign => {
                        modal_state.hide();
                        // TODO: Navigate to ship design screen
                        info!("Open ship design (not yet implemented)");
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(base_color.lighter(0.1));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(base_color);
            }
        }
    }
}

/// Public function to show a notification on the galaxy map.
/// Call this when events happen (e.g., construction complete, research done).
pub fn show_notification(modal_state: &mut InfoModalState, icon: ModalIcon, message: impl Into<String>) {
    *modal_state = InfoModalState::notification(icon, message);
}

/// Public function to show a planet-related notification.
pub fn show_planet_notification(
    modal_state: &mut InfoModalState,
    icon: ModalIcon,
    message: impl Into<String>,
    system_index: usize,
    planet_index: usize,
) {
    *modal_state = InfoModalState::planet_notification(icon, message, system_index, planet_index);
}
