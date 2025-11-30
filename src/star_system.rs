//! Star system view screen inspired by classic Ascendancy.
//!
//! Displays a 3D isometric view of a planetary system with:
//! - Planets on vertical poles/stalks
//! - Blue grid plane for depth reference
//! - Right-side control panel with navigation buttons
//! - Planet info display when selected

use bevy::{ecs::hierarchy::ChildSpawnerCommands, prelude::*};
use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::galaxy::Galaxy;
use crate::main_menu::GameState;

/// Plugin that manages the star system view screen.
pub struct StarSystemPlugin;

impl Plugin for StarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSystemState>()
            .add_systems(OnEnter(GameState::StarSystem), setup_star_system)
            .add_systems(OnExit(GameState::StarSystem), cleanup_star_system)
            .add_systems(
                Update,
                (
                    planet_hover_system,
                    panel_button_system,
                    camera_control_system,
                    keyboard_navigation_system,
                )
                    .run_if(in_state(GameState::StarSystem)),
            );
    }
}

/// Marker component for all star system view entities.
#[derive(Component)]
pub struct StarSystemRoot;

/// Marker for planet entities in the system view.
#[derive(Component)]
pub struct PlanetMarker {
    pub planet_index: usize,
}

/// Marker for planet selection ring.
#[derive(Component)]
pub struct SelectionRing;

/// Marker for the planet stalk/pole.
#[derive(Component)]
pub struct PlanetStalk {
    pub planet_index: usize,
}

/// Marker for the grid plane.
#[derive(Component)]
pub struct GridPlane;

/// State for the star system view.
#[derive(Resource, Default)]
pub struct StarSystemState {
    /// Currently viewing system index.
    pub system_index: usize,
    /// Currently selected planet (if any).
    pub selected_planet: Option<usize>,
    /// Camera rotation angle (for view rotation).
    pub camera_angle: f32,
    /// Camera zoom level.
    pub zoom: f32,
}

/// Side panel button types for system view.
#[derive(Component, Clone, Copy, Debug)]
pub enum SystemPanelButton {
    /// Navigate to planet (zoom in).
    GotoPlanet,
    /// Send fleet to system.
    SendFleet,
    /// Build ship order.
    BuildShip,
    /// View system info.
    SystemInfo,
    /// Return to galaxy map.
    ReturnToGalaxy,
}

/// Colors for the star system UI.
pub mod colors {
    use bevy::prelude::*;

    /// Black space background.
    pub const SPACE_BLACK: Color = Color::srgb(0.0, 0.0, 0.05);
    /// Grid line color (cyan/teal).
    pub const GRID_LINE: Color = Color::srgb(0.1, 0.4, 0.5);
    /// Grid line highlight.
    pub const GRID_HIGHLIGHT: Color = Color::srgb(0.15, 0.5, 0.6);
    /// Planet stalk/pole color (yellow-green).
    pub const STALK_COLOR: Color = Color::srgb(0.7, 0.8, 0.2);
    /// Selection ring color (bright green).
    pub const SELECTION_GREEN: Color = Color::srgb(0.2, 0.9, 0.3);
    /// Panel background (textured gray-green).
    pub const PANEL_BG: Color = Color::srgb(0.35, 0.42, 0.40);
    /// Panel dark sections.
    pub const PANEL_DARK: Color = Color::srgb(0.22, 0.28, 0.26);
    /// Panel border.
    pub const PANEL_BORDER: Color = Color::srgb(0.18, 0.22, 0.20);
    /// Text on panels.
    pub const PANEL_TEXT: Color = Color::srgb(0.85, 0.90, 0.85);
    /// Dim text.
    pub const PANEL_TEXT_DIM: Color = Color::srgb(0.55, 0.60, 0.55);
    /// Star label text (cyan).
    pub const STAR_LABEL: Color = Color::srgb(0.4, 0.9, 0.8);
    /// Planet name text.
    pub const PLANET_LABEL: Color = Color::srgb(0.3, 0.85, 0.7);
    /// Button icon backgrounds.
    pub const BUTTON_ICON_BG: Color = Color::srgb(0.25, 0.35, 0.45);
}

/// Planet type visual appearance.
#[derive(Clone, Copy, Debug)]
pub enum PlanetVisual {
    Rocky,      // Gray/brown
    Volcanic,   // Red/orange with glow
    Oceanic,    // Blue with white clouds
    Desert,     // Tan/yellow
    Lush,       // Green with blue
    Ice,        // White/light blue
    Gas,        // Striped bands
}

impl PlanetVisual {
    fn primary_color(&self) -> Color {
        match self {
            PlanetVisual::Rocky => Color::srgb(0.55, 0.45, 0.40),
            PlanetVisual::Volcanic => Color::srgb(0.75, 0.35, 0.20),
            PlanetVisual::Oceanic => Color::srgb(0.25, 0.45, 0.70),
            PlanetVisual::Desert => Color::srgb(0.80, 0.65, 0.40),
            PlanetVisual::Lush => Color::srgb(0.30, 0.60, 0.35),
            PlanetVisual::Ice => Color::srgb(0.85, 0.90, 0.95),
            PlanetVisual::Gas => Color::srgb(0.70, 0.55, 0.45),
        }
    }

    fn from_surface_type(surface_id: &str) -> Self {
        match surface_id {
            "eden" | "congenial" | "primordial" => PlanetVisual::Lush,
            "mineral" | "supermineral" => PlanetVisual::Rocky,
            "tycoon" => PlanetVisual::Desert,
            "husk" => PlanetVisual::Volcanic,
            "gas_giant" => PlanetVisual::Gas,
            _ => PlanetVisual::Rocky,
        }
    }
}

/// Generated position data for a planet in the system view.
struct PlanetPosition {
    /// Position on the grid (X, Z in 3D terms, mapped to 2D isometric).
    grid_pos: Vec2,
    /// Height above the grid (Y in 3D).
    height: f32,
    /// Visual size based on planet size.
    size: f32,
    /// Planet visual type.
    visual: PlanetVisual,
}

/// Generate planet positions for isometric view from galaxy data.
fn generate_planet_positions(galaxy: &Galaxy, system_index: usize, seed: u64) -> Vec<PlanetPosition> {
    let Some(system) = galaxy.systems.get(system_index) else {
        return Vec::new();
    };

    let mut rng = StdRng::seed_from_u64(seed.wrapping_add(system_index as u64));
    let mut positions = Vec::with_capacity(system.planets.len());

    let base_radius = 120.0;

    for (i, planet) in system.planets.iter().enumerate() {
        // Distribute planets in a spiral pattern
        let angle = (i as f32 / system.planets.len().max(1) as f32) * std::f32::consts::TAU
            + Rng::gen_range(&mut rng, -0.3..0.3);
        let distance = base_radius + (i as f32 * 60.0) + Rng::gen_range(&mut rng, -20.0..20.0);

        // Convert to isometric 2D coordinates
        let grid_x = angle.cos() * distance;
        let grid_z = angle.sin() * distance * 0.5; // Compress Z for isometric effect

        // Height varies by planet position
        let height = 80.0 + Rng::gen_range(&mut rng, -40.0..60.0);

        // Size based on planet surface slots
        let size = match planet.surface_slots {
            0..=10 => 20.0,
            11..=30 => 30.0,
            31..=50 => 40.0,
            _ => 50.0,
        };

        let visual = PlanetVisual::from_surface_type(&planet.surface_type_id);

        positions.push(PlanetPosition {
            grid_pos: Vec2::new(grid_x, grid_z),
            height,
            size,
            visual,
        });
    }

    positions
}

fn setup_star_system(
    mut commands: Commands,
    galaxy_preview: Res<crate::GalaxyPreview>,
    state: Res<StarSystemState>,
) {
    // Camera for the system view
    commands.spawn((Camera2d::default(), StarSystemRoot));

    // Background (space)
    commands.spawn((
        Sprite {
            color: colors::SPACE_BLACK,
            custom_size: Some(Vec2::new(2000.0, 2000.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
        StarSystemRoot,
    ));

    // Draw the grid plane
    spawn_grid_plane(&mut commands);

    // Generate and spawn planets with stalks
    let positions = generate_planet_positions(&galaxy_preview.galaxy, state.system_index, 42);

    // Get system info for the panel
    let system = galaxy_preview.galaxy.systems.get(state.system_index);
    let system_name = system.map(|s| s.name.as_str()).unwrap_or("Unknown System");

    for (i, pos) in positions.iter().enumerate() {
        let planet = system.and_then(|s| s.planets.get(i));

        // Calculate isometric screen position
        // X stays as X, Y combines height and Z depth
        let screen_x = pos.grid_pos.x;
        let screen_y = pos.grid_pos.y + pos.height;

        // Draw the stalk (vertical line from grid to planet)
        let stalk_base_y = pos.grid_pos.y;
        let stalk_height = pos.height;

        // Stalk sprite (stretched vertical line)
        commands.spawn((
            Sprite {
                color: colors::STALK_COLOR,
                custom_size: Some(Vec2::new(3.0, stalk_height)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                screen_x,
                stalk_base_y + stalk_height / 2.0,
                0.5,
            )),
            PlanetStalk { planet_index: i },
            StarSystemRoot,
        ));

        // Small base marker on grid
        commands.spawn((
            Sprite {
                color: colors::STALK_COLOR.with_alpha(0.8),
                custom_size: Some(Vec2::splat(8.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(screen_x, stalk_base_y, 0.4)),
            StarSystemRoot,
        ));

        // Planet sphere (represented as circle)
        commands.spawn((
            Sprite {
                color: pos.visual.primary_color(),
                custom_size: Some(Vec2::splat(pos.size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(screen_x, screen_y, 1.0)),
            PlanetMarker { planet_index: i },
            StarSystemRoot,
        ));

        // Planet highlight/atmosphere glow
        commands.spawn((
            Sprite {
                color: pos.visual.primary_color().with_alpha(0.3),
                custom_size: Some(Vec2::splat(pos.size * 1.3)),
                ..default()
            },
            Transform::from_translation(Vec3::new(screen_x, screen_y, 0.9)),
            StarSystemRoot,
        ));

        // Planet label (name)
        if let Some(p) = planet {
            let planet_name = format!("{} {}", system_name.replace("System-", ""), to_roman(i + 1));
            commands.spawn((
                Text::new(&planet_name),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::PLANET_LABEL),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(screen_x + 400.0 + pos.size / 2.0 + 10.0), // Offset from center
                    top: Val::Px(300.0 - screen_y - 5.0), // Invert Y for UI
                    ..default()
                },
                StarSystemRoot,
            ));
        }
    }

    // Add central star/galaxy indicator
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.4).with_alpha(0.9),
            custom_size: Some(Vec2::splat(60.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-50.0, 80.0, 0.8)),
        StarSystemRoot,
    ));

    // Star glow
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.9, 0.5).with_alpha(0.4),
            custom_size: Some(Vec2::splat(100.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-50.0, 80.0, 0.7)),
        StarSystemRoot,
    ));

    // Spawn background stars
    spawn_background_stars(&mut commands);

    // UI overlay - right side panel
    spawn_ui_panel(&mut commands, &galaxy_preview.galaxy, state.system_index);

    // System name in top-left
    spawn_system_label(&mut commands, system_name);

    // Instructions at bottom
    commands.spawn((
        Text::new("Click planets to select • Arrow keys to rotate • ESC for galaxy map"),
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
        StarSystemRoot,
    ));
}

/// Draw the isometric grid plane.
fn spawn_grid_plane(commands: &mut Commands) {
    let grid_size = 12;
    let cell_size = 40.0;
    let offset_x = -(grid_size as f32 * cell_size) / 2.0;
    let offset_y = -150.0; // Below center

    // Draw grid lines (simplified isometric - horizontal lines with slight perspective)
    for i in 0..=grid_size {
        let y_offset = i as f32 * (cell_size * 0.5);
        let x_start = offset_x - i as f32 * 10.0;
        let x_end = -offset_x + i as f32 * 10.0;

        // Horizontal-ish lines (going "into" the screen)
        let line_color = if i == grid_size / 2 {
            colors::GRID_HIGHLIGHT
        } else {
            colors::GRID_LINE
        };

        commands.spawn((
            Sprite {
                color: line_color.with_alpha(0.6),
                custom_size: Some(Vec2::new((x_end - x_start).abs(), 1.5)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                (x_start + x_end) / 2.0,
                offset_y + y_offset,
                0.1,
            )),
            GridPlane,
            StarSystemRoot,
        ));
    }

    // Vertical-ish lines (going "across")
    for i in 0..=grid_size {
        let x_pos = offset_x + i as f32 * cell_size;
        let y_start = offset_y;
        let y_end = offset_y + grid_size as f32 * (cell_size * 0.5);

        let line_color = if i == grid_size / 2 {
            colors::GRID_HIGHLIGHT
        } else {
            colors::GRID_LINE
        };

        // Slight slant for perspective
        let slant = (i as f32 - grid_size as f32 / 2.0) * 0.8;

        commands.spawn((
            Sprite {
                color: line_color.with_alpha(0.5),
                custom_size: Some(Vec2::new(1.5, (y_end - y_start).abs())),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                x_pos + slant,
                (y_start + y_end) / 2.0,
                0.1,
            )),
            GridPlane,
            StarSystemRoot,
        ));
    }
}

/// Spawn background stars for atmosphere.
fn spawn_background_stars(commands: &mut Commands) {
    let mut rng = StdRng::seed_from_u64(999);

    for _ in 0..80 {
        let x = Rng::gen_range(&mut rng, -500.0..500.0);
        let y = Rng::gen_range(&mut rng, -350.0..350.0);
        let brightness = Rng::gen_range(&mut rng, 0.15..0.5);
        let size = Rng::gen_range(&mut rng, 1.0..2.5);

        commands.spawn((
            Sprite {
                color: Color::srgba(brightness, brightness, brightness * 1.1, 0.7),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, -5.0)),
            StarSystemRoot,
        ));
    }
}

/// Spawn the right-side UI panel.
fn spawn_ui_panel(commands: &mut Commands, galaxy: &Galaxy, system_index: usize) {
    let system = galaxy.systems.get(system_index);
    let system_name = system.map(|s| s.name.as_str()).unwrap_or("Unknown");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            StarSystemRoot,
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
                    // System name header
                    spawn_system_header(panel, system_name);

                    // Navigation button row (like in screenshot)
                    spawn_navigation_buttons(panel);

                    // Planet info area
                    spawn_planet_info_area(panel, system);

                    // Spacer
                    panel.spawn(Node {
                        flex_grow: 1.0,
                        ..default()
                    });

                    // Bottom control buttons
                    spawn_bottom_controls(panel);
                });
        });
}

fn spawn_system_header(panel: &mut ChildSpawnerCommands, system_name: &str) {
    panel
        .spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(8.0)),
                margin: UiRect::bottom(Val::Px(8.0)),
                border: UiRect::all(Val::Px(1.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::PANEL_BORDER),
        ))
        .with_children(|header| {
            header.spawn((
                Text::new(system_name),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::STAR_LABEL),
            ));

            header.spawn((
                Text::new("White Medium"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(colors::PANEL_TEXT_DIM),
            ));
        });
}

fn spawn_navigation_buttons(panel: &mut ChildSpawnerCommands) {
    // Row of icon buttons (like the screenshot shows)
    panel
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                margin: UiRect::bottom(Val::Px(8.0)),
                column_gap: Val::Px(4.0),
                ..default()
            },
        ))
        .with_children(|row| {
            // Navigation icons (simplified representations)
            for (icon, _button) in [
                ("🌍", SystemPanelButton::GotoPlanet),
                ("➡", SystemPanelButton::SendFleet),
                ("🔍", SystemPanelButton::SystemInfo),
                ("➤", SystemPanelButton::BuildShip),
            ] {
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(45.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BUTTON_ICON_BG),
                    BorderColor::all(colors::PANEL_BORDER),
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
        });

    // Second row of navigation icons
    panel
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                margin: UiRect::bottom(Val::Px(12.0)),
                column_gap: Val::Px(4.0),
                ..default()
            },
        ))
        .with_children(|row| {
            for icon in ["📍", "🚀", "⚙", "📋"] {
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(45.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BUTTON_ICON_BG),
                    BorderColor::all(colors::PANEL_BORDER),
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
        });
}

fn spawn_planet_info_area(panel: &mut ChildSpawnerCommands, system: Option<&crate::galaxy::StarSystem>) {
    // Planet preview area (large image-like box)
    panel
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(150.0),
                margin: UiRect::bottom(Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.08, 0.12)),
            BorderColor::all(colors::SELECTION_GREEN),
        ))
        .with_children(|preview| {
            // Planet name label
            if let Some(sys) = system {
                if let Some(planet) = sys.planets.first() {
                    preview.spawn((
                        Text::new(format!("{} I", sys.name.replace("System-", "Icarus"))),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(colors::PLANET_LABEL),
                    ));

                    // Placeholder planet representation
                    preview.spawn((
                        Node {
                            width: Val::Px(80.0),
                            height: Val::Px(80.0),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(
                            PlanetVisual::from_surface_type(&planet.surface_type_id).primary_color(),
                        ),
                        BorderRadius::all(Val::Percent(50.0)),
                    ));
                }
            }
        });
}

fn spawn_bottom_controls(panel: &mut ChildSpawnerCommands) {
    // Grid of circular control buttons (matching Ascendancy style)
    panel
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            margin: UiRect::top(Val::Px(8.0)),
            ..default()
        })
        .with_children(|grid| {
            // First row
            grid.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            })
            .with_children(|row| {
                for (icon, color) in [
                    ("−", Color::srgb(0.4, 0.5, 0.6)),
                    ("⚠", Color::srgb(0.8, 0.3, 0.2)),
                    ("▲", Color::srgb(0.85, 0.55, 0.25)),
                    ("+", Color::srgb(0.4, 0.5, 0.6)),
                ] {
                    spawn_circular_button(row, icon, color);
                }
            });

            // Second row
            grid.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            })
            .with_children(|row| {
                for (icon, color) in [
                    ("⌀", Color::srgb(0.6, 0.4, 0.3)),
                    ("☆", Color::srgb(0.7, 0.6, 0.2)),
                    ("◎", Color::srgb(0.45, 0.55, 0.65)),
                    ("◈", Color::srgb(0.35, 0.45, 0.55)),
                ] {
                    spawn_circular_button(row, icon, color);
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
            BackgroundColor(bg_color.with_alpha(0.85)),
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

fn spawn_system_label(commands: &mut Commands, system_name: &str) {
    // Player species icon in corner
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
            BorderColor::all(colors::SELECTION_GREEN),
            StarSystemRoot,
        ))
        .with_children(|icon| {
            icon.spawn((
                Text::new("⬡"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::SELECTION_GREEN),
            ));
        });

    // Same icon in top-right (mirrored like in Ascendancy)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(230.0), // Account for panel width
                top: Val::Px(10.0),
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::PANEL_DARK),
            BorderColor::all(colors::SELECTION_GREEN),
            StarSystemRoot,
        ))
        .with_children(|icon| {
            icon.spawn((
                Text::new("⬡"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::SELECTION_GREEN),
            ));
        });
}

fn cleanup_star_system(mut commands: Commands, query: Query<Entity, With<StarSystemRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handle hovering over planets.
fn planet_hover_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<StarSystemRoot>>,
    mut planet_query: Query<(&PlanetMarker, &Transform, &mut Sprite)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<StarSystemState>,
    mut next_state: ResMut<NextState<GameState>>,
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

    let mut hovered_planet: Option<usize> = None;

    for (marker, transform, mut sprite) in &mut planet_query {
        let planet_pos = transform.translation.truncate();
        let size = sprite.custom_size.unwrap_or(Vec2::splat(30.0)).x;
        let distance = world_position.distance(planet_pos);

        if distance < size / 2.0 + 10.0 {
            hovered_planet = Some(marker.planet_index);
            // Highlight on hover
            sprite.color = sprite.color.with_alpha(1.0);
        } else {
            sprite.color = sprite.color.with_alpha(0.9);
        }
    }

    // Click to select planet and navigate to planet view
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(idx) = hovered_planet {
            state.selected_planet = Some(idx);
            info!("Selected planet {}, opening planet view", idx);
            next_state.set(GameState::PlanetView);
        }
    }
}

/// Handle panel button interactions.
fn panel_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(bg_color.0.with_alpha(1.0));
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(bg_color.0.with_alpha(0.95));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(bg_color.0.with_alpha(0.85));
            }
        }
    }
}

/// Handle camera/view controls.
fn camera_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, With<StarSystemRoot>)>,
    mut state: ResMut<StarSystemState>,
) {
    let mut delta = Vec2::ZERO;
    let rotation_speed = 2.0;

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        delta.x -= rotation_speed;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        delta.x += rotation_speed;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        delta.y += rotation_speed;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        delta.y -= rotation_speed;
    }

    if delta != Vec2::ZERO {
        for mut transform in &mut camera_query {
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
        }
    }

    // Zoom controls
    if keyboard.pressed(KeyCode::Equal) || keyboard.pressed(KeyCode::NumpadAdd) {
        state.zoom = (state.zoom + 0.02).min(2.0);
    }
    if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
        state.zoom = (state.zoom - 0.02).max(0.5);
    }
}

/// Handle keyboard navigation (ESC to return to galaxy map).
fn keyboard_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Returning to galaxy map...");
        next_state.set(GameState::InGame);
    }
}

/// Convert number to Roman numerals for planet names.
fn to_roman(n: usize) -> &'static str {
    match n {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        10 => "X",
        _ => "?",
    }
}
