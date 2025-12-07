//! Planet view screen implementation inspired by classic Ascendancy.
//!
//! Displays a 3D rotating planet globe with surface tiles rendered on the sphere,
//! buildings, population, and orbital structures. Accessed by clicking on a planet
//! in the star system view.

use bevy::camera::ScalingMode;
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

use crate::{
    main_menu::GameState,
    planet::{PlanetSurface, TileColor},
    star_system::StarSystemState,
    GalaxyPreview,
};

/// Plugin that manages the planet view screen.
pub struct PlanetViewPlugin;

impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlanetViewState>()
            .init_resource::<PlanetInfoModalState>()
            .add_systems(OnEnter(GameState::PlanetView), setup_planet_view)
            .add_systems(OnExit(GameState::PlanetView), cleanup_planet_view)
            .add_systems(
                Update,
                (
                    keyboard_navigation_system,
                    panel_button_system,
                    planet_info_modal_system,
                    planet_info_modal_button_system,
                )
                    .run_if(in_state(GameState::PlanetView)),
            );
    }
}

/// State for the planet view.
#[derive(Resource, Default)]
pub struct PlanetViewState {
    /// Index of the currently viewed planet within its star system.
    pub planet_index: usize,
}

/// Marker component for all planet view UI entities (2D overlay).
#[derive(Component)]
struct PlanetViewRoot;

/// Marker component for all 3D entities in the planet view.
#[derive(Component)]
struct PlanetView3D;

/// Marker for the planet grid mesh.
#[derive(Component)]
struct PlanetGrid;

/// Marker for planet thumbnail buttons in the top bar.
#[derive(Component)]
struct PlanetThumbnail(usize);

/// Marker for surface tiles (now on the 3D sphere).
#[derive(Component)]
struct SurfaceTileMarker {
    index: usize,
    color: TileColor,
}

/// Marker for tile cube on the planet surface.
#[derive(Component)]
struct TileCube {
    tile_index: usize,
}

/// Marker for the population display.
#[derive(Component)]
struct PopulationDisplay;

/// Marker for the project display.
#[derive(Component)]
struct ProjectDisplay;

/// Marker for the back button.
#[derive(Component)]
struct BackButton;

/// Panel button types.
#[derive(Component, Clone, Copy, Debug)]
enum PanelButton {
    Back,
}

// ============================================================================
// Planet Info Modal
// ============================================================================

/// State for the planet info modal overlay.
#[derive(Resource, Default)]
pub struct PlanetInfoModalState {
    /// Whether the modal is visible.
    pub visible: bool,
    /// Planet name to display.
    pub planet_name: String,
    /// Prosperity rate per day.
    pub prosperity_per_day: i32,
    /// Days until next population growth.
    pub days_to_growth: i32,
    /// Current population count.
    pub population: i32,
    /// Maximum population capacity.
    pub max_population: i32,
}

impl PlanetInfoModalState {
    /// Show the modal with planet info.
    pub fn show(&mut self, name: impl Into<String>, prosperity: i32, days: i32, pop: i32, max_pop: i32) {
        self.visible = true;
        self.planet_name = name.into();
        self.prosperity_per_day = prosperity;
        self.days_to_growth = days;
        self.population = pop;
        self.max_population = max_pop;
    }

    /// Hide the modal.
    pub fn hide(&mut self) {
        self.visible = false;
    }
}

/// Marker for the planet info modal overlay.
#[derive(Component)]
struct PlanetInfoModalOverlay;

/// Marker for the modal OK button.
#[derive(Component)]
struct PlanetInfoModalButton;

/// Colors for the planet view UI - inspired by Ascendancy's planet screen.
mod colors {
    use bevy::prelude::*;

    /// Dark space background.
    pub const BACKGROUND: Color = Color::srgb(0.02, 0.02, 0.05);
    /// Panel background.
    pub const PANEL_BG: Color = Color::srgb(0.08, 0.10, 0.15);
    /// Border color - teal/cyan accent.
    pub const BORDER: Color = Color::srgb(0.2, 0.5, 0.6);
    /// Header text color.
    pub const HEADER_TEXT: Color = Color::srgb(0.7, 0.85, 0.9);
    /// Normal text color.
    pub const TEXT: Color = Color::srgb(0.6, 0.7, 0.75);
    /// Button normal.
    pub const BUTTON_NORMAL: Color = Color::srgb(0.08, 0.12, 0.20);
    /// Button hovered.
    pub const BUTTON_HOVERED: Color = Color::srgb(0.12, 0.18, 0.28);

    // Tile colors matching TileColor enum
    /// Black/hostile terrain - impassable.
    pub const TILE_BLACK: Color = Color::srgb(0.1, 0.1, 0.1);
    /// White/marginal terrain - buildable but plain.
    pub const TILE_WHITE: Color = Color::srgb(0.75, 0.75, 0.7);
    /// Red/mineral terrain - industry bonus.
    pub const TILE_RED: Color = Color::srgb(0.8, 0.3, 0.2);
    /// Green/fertile terrain - prosperity bonus.
    pub const TILE_GREEN: Color = Color::srgb(0.3, 0.7, 0.3);
    /// Blue/special terrain - research bonus.
    pub const TILE_BLUE: Color = Color::srgb(0.3, 0.5, 0.8);

    /// Tile border.
    pub const TILE_BORDER: Color = Color::srgb(0.3, 0.3, 0.35);
    /// Tile hover highlight.
    pub const TILE_HOVER: Color = Color::srgb(0.9, 0.8, 0.3);

    /// Selected planet thumbnail border.
    pub const THUMBNAIL_SELECTED: Color = Color::srgb(0.9, 0.7, 0.2);
    /// Unselected planet thumbnail border.
    pub const THUMBNAIL_NORMAL: Color = Color::srgb(0.3, 0.4, 0.5);
}

fn setup_planet_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    star_system_state: Res<StarSystemState>,
    galaxy_preview: Res<GalaxyPreview>,
    mut planet_state: ResMut<PlanetViewState>,
) {
    // Get current planet info from star system state
    let system_index = star_system_state.system_index;
    let planet_index = star_system_state.selected_planet.unwrap_or(0);

    // Get planet data if available
    let (planet_name, surface_type, planet_size, surface_slots, orbital_slots, tiles, row_width) = galaxy_preview
        .galaxy
        .systems
        .get(system_index)
        .and_then(|s| s.planets.get(planet_index))
        .map(|p| {
            let _surface = PlanetSurface::from(p);
            (
                format!("Planet {}", planet_index + 1),
                p.surface_type_id.clone(),
                p.size_id.clone(),
                p.surface_slots,
                p.orbital_slots,
                p.tiles.clone(),
                p.row_width,
            )
        })
        .unwrap_or_else(|| {
            (
                "Unknown Planet".to_string(),
                "unknown".to_string(),
                "unknown".to_string(),
                0,
                0,
                Vec::new(),
                1,
            )
        });

    let num_planets = galaxy_preview
        .galaxy
        .systems
        .get(system_index)
        .map(|s| s.planets.len())
        .unwrap_or(0);

    // =========================================================================
    // 3D Scene Setup
    // =========================================================================
    
    // Isometric Camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 15.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Camera {
            order: 0, // Render first (background)
            clear_color: ClearColorConfig::Custom(colors::BACKGROUND),
            ..default()
        },
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlanetView3D,
    ));

    // Ambient light for base illumination
    commands.spawn((
        AmbientLight {
            color: Color::WHITE,
            brightness: 300.0,
            ..default()
        },
        PlanetView3D,
    ));

    // Directional light (sun-like)
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlanetView3D,
    ));

    // Create planet grid with colored tiles
    let grid_mesh = create_planet_grid_mesh(&mut meshes, &tiles, row_width);
    let planet_material = create_planet_material(&mut materials, &surface_type);

    // Planet grid entity
    commands.spawn((
        Mesh3d(grid_mesh),
        MeshMaterial3d(planet_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        PlanetGrid,
        PlanetView3D,
    ));

    // Spawn building cubes on the surface (for tiles that have special features)
    spawn_surface_buildings(
        &mut commands,
        &mut meshes,
        &mut materials,
        &tiles,
        row_width,
    );

    // =========================================================================
    // 2D UI Overlay
    // =========================================================================
    
    // 2D Camera for UI overlay
    commands.spawn((
        Camera2d,
        Camera {
            order: 1, // Render on top
            clear_color: ClearColorConfig::None,
            ..default()
        },
        PlanetViewRoot,
    ));

    // Root container - full screen UI overlay
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            // Transparent background - 3D scene shows through
            BackgroundColor(Color::NONE),
            PlanetViewRoot,
        ))
        .with_children(|root| {
            // Top bar - planet thumbnails and info
            spawn_top_bar(root, num_planets, planet_index, system_index, &galaxy_preview, &planet_name, &surface_type, &planet_size);

            // Main content area - sides only (center is transparent for 3D)
            root.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            })
            .with_children(|main| {
                // Left panel - Planet info
                spawn_left_panel(main, &planet_name, &surface_type, &planet_size, surface_slots as usize, orbital_slots as usize);

                // Center area - transparent (3D shows through)
                main.spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                });

                // Right panel - Orbital structures
                spawn_right_panel(main, orbital_slots as usize);
            });
        });
}

// =============================================================================
// 3D Helper Functions
// =============================================================================

/// Create a planet grid mesh with vertex colors based on tiles.
fn create_planet_grid_mesh(
    meshes: &mut ResMut<Assets<Mesh>>,
    tiles: &[TileColor],
    row_width: usize,
) -> Handle<Mesh> {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList, 
        bevy_asset::RenderAssetUsages::default()
    );
    
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let tile_size = 1.0;
    let gap = 0.05; // Small gap between tiles
    let half_size = (tile_size - gap) / 2.0;

    let rows = (tiles.len() + row_width - 1) / row_width;
    let offset_x = -((row_width as f32) * tile_size) / 2.0;
    let offset_z = -((rows as f32) * tile_size) / 2.0;

    for (i, tile) in tiles.iter().enumerate() {
        // Skip black tiles (void)
        if *tile == TileColor::Black {
            continue;
        }

        let x_idx = i % row_width;
        let z_idx = i / row_width;

        let cx = offset_x + (x_idx as f32 * tile_size) + tile_size / 2.0;
        let cz = offset_z + (z_idx as f32 * tile_size) + tile_size / 2.0;
        let cy = 0.0;

        let base_index = positions.len() as u32;

        // 4 vertices for the quad
        positions.push([cx - half_size, cy, cz - half_size]);
        positions.push([cx + half_size, cy, cz - half_size]);
        positions.push([cx + half_size, cy, cz + half_size]);
        positions.push([cx - half_size, cy, cz + half_size]);

        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);

        let color = tile_color_to_linear(*tile);
        colors.push(color);
        colors.push(color);
        colors.push(color);
        colors.push(color);

        // 2 triangles
        indices.push(base_index);
        indices.push(base_index + 2);
        indices.push(base_index + 1);
        
        indices.push(base_index);
        indices.push(base_index + 3);
        indices.push(base_index + 2);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(bevy_mesh::Indices::U32(indices));
    
    meshes.add(mesh)
}

/// Create a material for the planet.
fn create_planet_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    surface_type: &str,
) -> Handle<StandardMaterial> {
    let base_color = get_planet_base_color(surface_type);
    
    materials.add(StandardMaterial {
        base_color,
        // Use vertex colors
        base_color_texture: None,
        perceptual_roughness: 0.7,
        metallic: 0.0,
        reflectance: 0.3,
        ..default()
    })
}

/// Get the base color for a planet type.
fn get_planet_base_color(surface_type: &str) -> Color {
    match surface_type {
        "husk" => Color::srgb(0.2, 0.15, 0.15),
        "primordial" => Color::srgb(0.5, 0.45, 0.4),
        "congenial" => Color::srgb(0.6, 0.65, 0.6),
        "eden" => Color::srgb(0.3, 0.6, 0.35),
        "mineral" => Color::srgb(0.6, 0.4, 0.35),
        "supermineral" => Color::srgb(0.7, 0.35, 0.3),
        "chapel" | "cathedral" => Color::srgb(0.4, 0.5, 0.7),
        "special" => Color::srgb(0.6, 0.5, 0.65),
        "tycoon" => Color::srgb(0.75, 0.65, 0.4),
        "cornucopia" => Color::srgb(0.8, 0.7, 0.5),
        _ => Color::srgb(0.6, 0.6, 0.55),
    }
}

/// Spawn building cubes on special tiles.
fn spawn_surface_buildings(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    tiles: &[TileColor],
    row_width: usize,
) {
    // Create a small cube mesh for buildings
    let cube_mesh = meshes.add(Cuboid::new(0.6, 0.6, 0.6));
    
    // Create materials for different building types
    let industry_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.5, 0.2),
        metallic: 0.3,
        ..default()
    });
    let research_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.5, 0.8),
        emissive: bevy::color::LinearRgba::rgb(0.1, 0.2, 0.4),
        ..default()
    });
    let prosperity_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.7, 0.35),
        ..default()
    });

    let tile_size = 1.0;
    let rows = (tiles.len() + row_width - 1) / row_width;
    let offset_x = -((row_width as f32) * tile_size) / 2.0;
    let offset_z = -((rows as f32) * tile_size) / 2.0;
    
    // Place buildings on colored (non-white, non-black) tiles
    for (i, tile) in tiles.iter().enumerate() {
        let mat = match tile {
            TileColor::Red => Some(industry_mat.clone()),
            TileColor::Blue => Some(research_mat.clone()),
            TileColor::Green => Some(prosperity_mat.clone()),
            _ => None,
        };
        
        if let Some(material) = mat {
            let x_idx = i % row_width;
            let z_idx = i / row_width;

            let cx = offset_x + (x_idx as f32 * tile_size) + tile_size / 2.0;
            let cz = offset_z + (z_idx as f32 * tile_size) + tile_size / 2.0;
            
            let pos = Vec3::new(cx, 0.3, cz); // Center of 0.6 height cube is at 0.3
            
            commands.spawn((
                Mesh3d(cube_mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(pos),
                TileCube { tile_index: i },
                PlanetView3D,
            ));
        }
    }
}

/// Convert TileColor to linear RGBA for vertex colors.
fn tile_color_to_linear(tile: TileColor) -> [f32; 4] {
    match tile {
        TileColor::Black => [0.1, 0.1, 0.1, 1.0],
        TileColor::White => [0.75, 0.75, 0.7, 1.0],
        TileColor::Red => [0.8, 0.3, 0.2, 1.0],
        TileColor::Green => [0.3, 0.7, 0.3, 1.0],
        TileColor::Blue => [0.3, 0.5, 0.8, 1.0],
    }
}

/// Convert Color to linear RGBA array.
fn color_to_linear(color: Color) -> [f32; 4] {
    let srgba = color.to_srgba();
    [srgba.red, srgba.green, srgba.blue, srgba.alpha]
}

// =============================================================================
// UI Panel Spawning Functions
// =============================================================================

fn spawn_top_bar(
    root: &mut ChildSpawnerCommands,
    num_planets: usize,
    planet_index: usize,
    system_index: usize,
    galaxy_preview: &GalaxyPreview,
    planet_name: &str,
    surface_type: &str,
    planet_size: &str,
) {
    root.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(80.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(10.0)),
            column_gap: Val::Px(8.0),
            border: UiRect::bottom(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(colors::PANEL_BG.with_alpha(0.9)),
        BorderColor::all(colors::BORDER),
    ))
    .with_children(|top_bar| {
        // Left section: Back button
        top_bar
            .spawn((
                Button,
                Node {
                    width: Val::Px(60.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(colors::BUTTON_NORMAL),
                BorderColor::all(colors::BORDER),
                PanelButton::Back,
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new("◀"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(colors::HEADER_TEXT),
                ));
            });

        // Center section: Planet info
        top_bar.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|info| {
            info.spawn((
                Text::new(planet_name),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::HEADER_TEXT),
            ));
            info.spawn((
                Text::new(format!("{} • {}", surface_type, planet_size)),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT),
            ));
        });

        // Right section: Planet thumbnails
        top_bar.spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(5.0),
            ..default()
        }).with_children(|thumbs| {
            for i in 0..num_planets {
                let is_selected = i == planet_index;
                let border_color = if is_selected {
                    colors::THUMBNAIL_SELECTED
                } else {
                    colors::THUMBNAIL_NORMAL
                };

                let thumb_color = galaxy_preview
                    .galaxy
                    .systems
                    .get(system_index)
                    .and_then(|s| s.planets.get(i))
                    .map(|p| get_planet_thumbnail_color(&p.surface_type_id))
                    .unwrap_or(colors::TILE_WHITE);

                thumbs
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(thumb_color),
                        BorderColor::all(border_color),
                        PlanetThumbnail(i),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(format!("{}", i + 1)),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
    });
}

fn spawn_left_panel(
    main: &mut ChildSpawnerCommands,
    planet_name: &str,
    surface_type: &str,
    planet_size: &str,
    surface_slots: usize,
    orbital_slots: usize,
) {
    main.spawn((
        Node {
            width: Val::Px(220.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            row_gap: Val::Px(10.0),
            border: UiRect::right(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(colors::PANEL_BG.with_alpha(0.85)),
        BorderColor::all(colors::BORDER),
    ))
    .with_children(|panel| {
        // Surface info header
        panel.spawn((
            Text::new("Surface"),
            TextFont {
                font_size: 22.0,
                ..default()
            },
            TextColor(colors::HEADER_TEXT),
        ));

        // Stats
        for (label, value) in [
            ("Slots", format!("{}", surface_slots)),
            ("Orbitals", format!("{}", orbital_slots)),
        ] {
            panel.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            }).with_children(|row| {
                row.spawn((
                    Text::new(label),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(colors::TEXT),
                ));
                row.spawn((
                    Text::new(value),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(colors::HEADER_TEXT),
                ));
            });
        }

        // Divider
        panel.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(colors::BORDER),
        ));

        // Population section
        panel.spawn((
            Text::new("Population"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::HEADER_TEXT),
        ));

        panel.spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(4.0),
            ..default()
        }).with_children(|pop_row| {
            for _ in 0..3 {
                pop_row.spawn((
                    Node {
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.6, 0.9)),
                ));
            }
        });

        // Divider
        panel.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                margin: UiRect::vertical(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(colors::BORDER),
        ));

        // Project section
        panel.spawn((
            Text::new("Project"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::HEADER_TEXT),
        ));
        panel.spawn((
            Text::new("None"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT.with_alpha(0.6)),
        ));

        // Controls at bottom
        panel.spawn((
            Node {
                margin: UiRect::top(Val::Auto),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },
        )).with_children(|controls| {
            controls.spawn((
                Text::new("ESC - Return"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(colors::TEXT.with_alpha(0.5)),
            ));
        });
    });
}

fn spawn_right_panel(main: &mut ChildSpawnerCommands, orbital_slots: usize) {
    main.spawn((
        Node {
            width: Val::Px(180.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            row_gap: Val::Px(8.0),
            border: UiRect::left(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(colors::PANEL_BG.with_alpha(0.85)),
        BorderColor::all(colors::BORDER),
    ))
    .with_children(|panel| {
        panel.spawn((
            Text::new("Orbitals"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(colors::HEADER_TEXT),
        ));

        // Orbital slots display
        for i in 0..orbital_slots.min(8) {
            panel
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(28.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(colors::BUTTON_NORMAL),
                    BorderColor::all(colors::BORDER),
                ))
                .with_children(|slot| {
                    slot.spawn((
                        Text::new(format!("Slot {}", i + 1)),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(colors::TEXT.with_alpha(0.5)),
                    ));
                });
        }

        if orbital_slots > 8 {
            panel.spawn((
                Text::new(format!("+{} more", orbital_slots - 8)),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(colors::TEXT.with_alpha(0.4)),
            ));
        }
    });
}

// =============================================================================
// Cleanup
// =============================================================================

fn cleanup_planet_view(
    mut commands: Commands,
    ui_query: Query<Entity, With<PlanetViewRoot>>,
    view_3d_query: Query<Entity, With<PlanetView3D>>,
) {
    for entity in &ui_query {
        commands.entity(entity).despawn();
    }
    for entity in &view_3d_query {
        commands.entity(entity).despawn();
    }
}

// =============================================================================
// Systems
// =============================================================================

/// Handle keyboard navigation - ESC returns to star system, I shows info modal.
fn keyboard_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut modal_state: ResMut<PlanetInfoModalState>,
    mut planet_state: ResMut<PlanetViewState>,
    star_system_state: Res<StarSystemState>,
    galaxy_preview: Res<GalaxyPreview>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if modal_state.visible {
            modal_state.hide();
        } else {
            next_state.set(GameState::StarSystem);
        }
    }

    // Press 'I' to show planet info modal
    if keyboard.just_pressed(KeyCode::KeyI) && !modal_state.visible {
        let system_index = star_system_state.system_index;
        let planet_index = star_system_state.selected_planet.unwrap_or(0);

        if let Some(system) = galaxy_preview.galaxy.systems.get(system_index) {
            if let Some(planet) = system.planets.get(planet_index) {
                // Generate planet name like setup does
                let planet_name = format!("Planet {}", planet_index + 1);
                
                // Calculate some mock values for prosperity
                let prosperity = 1; // 1 per day base
                let days_to_growth = 20; // Placeholder
                let population = 0; // No population tracking yet
                let max_pop = planet.surface_slots as i32;

                modal_state.show(
                    planet_name,
                    prosperity,
                    days_to_growth,
                    population,
                    max_pop,
                );
            }
        }
    }
}

/// Handle panel buttons (back, toggle rotation, thumbnails).
fn panel_button_system(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &PanelButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut thumbnail_query: Query<
        (&Interaction, &PlanetThumbnail, &mut BorderColor),
        (Changed<Interaction>, With<Button>, Without<PanelButton>),
    >,
    mut star_system_state: ResMut<StarSystemState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Panel buttons
    for (interaction, mut bg_color, button) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                match button {
                    PanelButton::Back => {
                        next_state.set(GameState::StarSystem);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(colors::BUTTON_HOVERED);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(colors::BUTTON_NORMAL);
            }
        }
    }

    // Planet thumbnail clicks
    for (interaction, thumbnail, mut border_color) in &mut thumbnail_query {
        if *interaction == Interaction::Pressed {
            star_system_state.selected_planet = Some(thumbnail.0);
            // Re-enter planet view to refresh
            next_state.set(GameState::PlanetView);
        } else if *interaction == Interaction::Hovered {
            *border_color = BorderColor::all(colors::THUMBNAIL_SELECTED.with_alpha(0.7));
        }
    }
}



/// Get a representative color for planet thumbnails based on surface type.
fn get_planet_thumbnail_color(surface_type: &str) -> Color {
    match surface_type {
        "husk" => colors::TILE_BLACK,
        "primordial" => Color::srgb(0.4, 0.35, 0.3),
        "congenial" => colors::TILE_WHITE,
        "eden" => colors::TILE_GREEN,
        "mineral" | "supermineral" => colors::TILE_RED,
        "chapel" | "cathedral" => colors::TILE_BLUE,
        "special" => Color::srgb(0.6, 0.5, 0.6),
        "tycoon" => Color::srgb(0.7, 0.6, 0.3),
        "cornucopia" => Color::srgb(0.8, 0.7, 0.5),
        _ => colors::TILE_WHITE,
    }
}

// ============================================================================
// Planet Info Modal Implementation
// ============================================================================

/// Spawn or despawn the planet info modal based on state.
fn planet_info_modal_system(
    mut commands: Commands,
    modal_state: Res<PlanetInfoModalState>,
    modal_query: Query<Entity, With<PlanetInfoModalOverlay>>,
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

    // Spawn the modal overlay
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            GlobalZIndex(100),
            PlanetInfoModalOverlay,
            PlanetViewRoot,
        ))
        .with_children(|parent| {
            // Modal panel with starfield-like dark background
            parent
                .spawn((
                    Node {
                        width: Val::Px(380.0),
                        min_height: Val::Px(220.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(0.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.02, 0.02, 0.06)),
                    BorderColor::all(colors::BORDER),
                    BorderRadius::all(Val::Px(4.0)),
                ))
                .with_child((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(3.0),
                        ..default()
                    },
                    BackgroundColor(colors::BORDER),
                    BorderRadius::top(Val::Px(4.0)),
                ))
                .with_children(|panel| {
                    // Main content area with starfield background effect
                    panel
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                padding: UiRect::new(
                                    Val::Px(30.0),
                                    Val::Px(30.0),
                                    Val::Px(35.0),
                                    Val::Px(25.0),
                                ),
                                row_gap: Val::Px(20.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                        ))
                        .with_children(|content| {
                            // Title line: "Icarus I Prosperity: 1 per day"
                            content.spawn((
                                Text::new(format!(
                                    "{} Prosperity: {} per day",
                                    modal_state.planet_name,
                                    modal_state.prosperity_per_day
                                )),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(colors::HEADER_TEXT),
                            ));

                            // Population growth info
                            let growth_text = if modal_state.days_to_growth > 0 {
                                format!("Population will grow in {} days.", modal_state.days_to_growth)
                            } else {
                                "Population at maximum capacity.".to_string()
                            };

                            content.spawn((
                                Text::new(growth_text),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(colors::TEXT),
                            ));

                            // Current population status
                            content.spawn((
                                Text::new(format!(
                                    "Population: {} / {}",
                                    modal_state.population,
                                    modal_state.max_population
                                )),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 0.6, 0.65)),
                            ));
                        });

                    // Bottom border line
                    panel.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(2.0),
                            ..default()
                        },
                        BackgroundColor(colors::BORDER),
                    ));

                    // OK button row
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(12.0)),
                            ..default()
                        })
                        .with_children(|button_row| {
                            button_row
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(36.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.15, 0.2, 0.25)),
                                    BorderColor::all(colors::BORDER),
                                    BorderRadius::all(Val::Px(3.0)),
                                    PlanetInfoModalButton,
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("OK"),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(colors::HEADER_TEXT),
                                    ));
                                });
                        });
                });
        });
}

/// Handle modal button clicks.
fn planet_info_modal_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlanetInfoModalButton>),
    >,
    mut modal_state: ResMut<PlanetInfoModalState>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                modal_state.hide();
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.28, 0.35));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.15, 0.2, 0.25));
            }
        }
    }
}
