//! Planet view screen implementation inspired by classic Ascendancy.
//!
//! Displays a planet's surface with isometric tile grid, buildings, population,
//! and orbital structures. Accessed by clicking on a planet in the star system view.

use bevy::prelude::*;

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
                    tile_hover_system,
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

/// Marker component for all planet view UI entities.
#[derive(Component)]
struct PlanetViewRoot;

/// Marker for planet thumbnail buttons in the top bar.
#[derive(Component)]
struct PlanetThumbnail(usize);

/// Marker for surface tiles.
#[derive(Component)]
struct SurfaceTileMarker {
    index: usize,
    color: TileColor,
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
    star_system_state: Res<StarSystemState>,
    galaxy_preview: Res<GalaxyPreview>,
) {
    // Camera for the planet view
    commands.spawn((Camera2d::default(), PlanetViewRoot));

    // Get current planet info from star system state
    let system_index = star_system_state.system_index;
    let planet_index = star_system_state.selected_planet.unwrap_or(0);

    // Get planet data if available
    let (planet_name, surface_type, planet_size, surface_slots, orbital_slots, tiles) = galaxy_preview
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
            )
        });

    let num_planets = galaxy_preview
        .galaxy
        .systems
        .get(system_index)
        .map(|s| s.planets.len())
        .unwrap_or(0);

    // Root container - full screen
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND),
            PlanetViewRoot,
        ))
        .with_children(|root| {
            // Top bar - planet thumbnails
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    column_gap: Val::Px(8.0),
                    border: UiRect::bottom(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(colors::PANEL_BG),
                BorderColor::all(colors::BORDER),
            ))
            .with_children(|top_bar| {
                // Back button
                top_bar
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(60.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            margin: UiRect::right(Val::Px(20.0)),
                            ..default()
                        },
                        BackgroundColor(colors::BUTTON_NORMAL),
                        BorderColor::all(colors::BORDER),
                        BackButton,
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

                // Planet thumbnails
                for i in 0..num_planets {
                    let is_selected = i == planet_index;
                    let border_color = if is_selected {
                        colors::THUMBNAIL_SELECTED
                    } else {
                        colors::THUMBNAIL_NORMAL
                    };

                    // Get planet surface type for color
                    let thumb_color = galaxy_preview
                        .galaxy
                        .systems
                        .get(system_index)
                        .and_then(|s| s.planets.get(i))
                        .map(|p| get_planet_thumbnail_color(&p.surface_type_id))
                        .unwrap_or(colors::TILE_WHITE);

                    top_bar
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(3.0)),
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
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                }
            });

            // Main content area
            root.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            })
            .with_children(|main| {
                // Left panel - Planet info
                main.spawn((
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        row_gap: Val::Px(15.0),
                        border: UiRect::right(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(colors::PANEL_BG),
                    BorderColor::all(colors::BORDER),
                ))
                .with_children(|panel| {
                    // Planet name header
                    panel.spawn((
                        Text::new(&planet_name),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(colors::HEADER_TEXT),
                    ));

                    // Planet type
                    panel.spawn((
                        Text::new(format!("Type: {}", surface_type)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::TEXT),
                    ));

                    // Planet size
                    panel.spawn((
                        Text::new(format!("Size: {}", planet_size)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::TEXT),
                    ));

                    // Surface slots
                    panel.spawn((
                        Text::new(format!("Surface: {} slots", surface_slots)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::TEXT),
                    ));

                    // Orbital slots
                    panel.spawn((
                        Text::new(format!("Orbital: {} slots", orbital_slots)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::TEXT),
                    ));

                    // Divider
                    panel.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(2.0),
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor(colors::BORDER),
                    ));

                    // Population section
                    panel.spawn((
                        Text::new("Population"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(colors::HEADER_TEXT),
                    ));

                    panel
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(5.0),
                                ..default()
                            },
                            PopulationDisplay,
                        ))
                        .with_children(|pop_row| {
                            // Colonist icons (placeholder - would show actual pop)
                            for _ in 0..3 {
                                pop_row.spawn((
                                    Node {
                                        width: Val::Px(20.0),
                                        height: Val::Px(20.0),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.4, 0.6, 0.9)),
                                ));
                            }
                        });

                    // Project section
                    panel.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(5.0),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                        ProjectDisplay,
                    ))
                    .with_children(|project| {
                        project.spawn((
                            Text::new("Project"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(colors::HEADER_TEXT),
                        ));
                        project.spawn((
                            Text::new("No Project"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(colors::TEXT),
                        ));
                    });

                    // Instructions
                    panel.spawn((
                        Text::new("ESC - Return to system"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(colors::TEXT.with_alpha(0.6)),
                        Node {
                            margin: UiRect::top(Val::Auto),
                            ..default()
                        },
                    ));
                });

                // Center area - Surface grid
                main.spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|center| {
                    // Surface grid container
                    let row_width = if tiles.is_empty() {
                        1
                    } else {
                        (tiles.len() as f32).sqrt().ceil() as usize
                    };

                    let tile_size = calculate_tile_size(tiles.len(), row_width);

                    center
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(2.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(colors::PANEL_BG.with_alpha(0.5)),
                            BorderColor::all(colors::BORDER),
                        ))
                        .with_children(|grid_container| {
                            // Create rows of tiles
                            let mut tile_iter = tiles.iter().enumerate().peekable();
                            
                            while tile_iter.peek().is_some() {
                                grid_container
                                    .spawn(Node {
                                        flex_direction: FlexDirection::Row,
                                        column_gap: Val::Px(2.0),
                                        ..default()
                                    })
                                    .with_children(|row| {
                                        for _ in 0..row_width {
                                            if let Some((idx, tile_color)) = tile_iter.next() {
                                                let bg_color = tile_color_to_color(*tile_color);
                                                row.spawn((
                                                    Button,
                                                    Node {
                                                        width: Val::Px(tile_size),
                                                        height: Val::Px(tile_size),
                                                        border: UiRect::all(Val::Px(1.0)),
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    BackgroundColor(bg_color),
                                                    BorderColor::all(colors::TILE_BORDER),
                                                    SurfaceTileMarker {
                                                        index: idx,
                                                        color: *tile_color,
                                                    },
                                                ));
                                            }
                                        }
                                    });
                            }

                            // If no tiles, show placeholder
                            if tiles.is_empty() {
                                grid_container.spawn((
                                    Text::new("No surface data"),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(colors::TEXT),
                                ));
                            }
                        });
                });

                // Right panel - Orbital structures
                main.spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(15.0)),
                        row_gap: Val::Px(10.0),
                        border: UiRect::left(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(colors::PANEL_BG),
                    BorderColor::all(colors::BORDER),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Text::new("Orbitals"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(colors::HEADER_TEXT),
                    ));

                    // Orbital slots display
                    for i in 0..orbital_slots.min(10) {
                        panel
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(30.0),
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
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(colors::TEXT.with_alpha(0.5)),
                                ));
                            });
                    }

                    if orbital_slots > 10 {
                        panel.spawn((
                            Text::new(format!("... +{} more", orbital_slots - 10)),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(colors::TEXT.with_alpha(0.5)),
                        ));
                    }
                });
            });
        });
}

fn cleanup_planet_view(mut commands: Commands, query: Query<Entity, With<PlanetViewRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handle keyboard navigation - ESC returns to star system, I shows info modal.
fn keyboard_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut modal_state: ResMut<PlanetInfoModalState>,
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

/// Handle tile hover effects and back button.
fn tile_hover_system(
    mut tile_query: Query<
        (&Interaction, &mut BorderColor, &SurfaceTileMarker),
        (Changed<Interaction>, With<Button>),
    >,
    mut back_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>, Without<SurfaceTileMarker>),
    >,
    mut thumbnail_query: Query<
        (&Interaction, &PlanetThumbnail, &mut BorderColor),
        (Changed<Interaction>, Without<SurfaceTileMarker>, Without<BackButton>),
    >,
    mut star_system_state: ResMut<StarSystemState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Tile hover effects
    for (interaction, mut border_color, _tile) in &mut tile_query {
        match *interaction {
            Interaction::Hovered | Interaction::Pressed => {
                *border_color = BorderColor::all(colors::TILE_HOVER);
            }
            Interaction::None => {
                *border_color = BorderColor::all(colors::TILE_BORDER);
            }
        }
    }

    // Back button
    for (interaction, mut bg_color) in &mut back_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::StarSystem);
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

/// Convert TileColor to Bevy Color.
fn tile_color_to_color(tile: TileColor) -> Color {
    match tile {
        TileColor::Black => colors::TILE_BLACK,
        TileColor::White => colors::TILE_WHITE,
        TileColor::Red => colors::TILE_RED,
        TileColor::Green => colors::TILE_GREEN,
        TileColor::Blue => colors::TILE_BLUE,
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

/// Calculate appropriate tile size based on grid dimensions.
fn calculate_tile_size(total_tiles: usize, row_width: usize) -> f32 {
    // Aim for a grid that fits nicely on screen
    // Max grid area: ~500x500 pixels
    let max_grid_size = 500.0;
    let num_rows = (total_tiles as f32 / row_width as f32).ceil();
    
    let max_tile_by_width = (max_grid_size - (row_width as f32 * 2.0)) / row_width as f32;
    let max_tile_by_height = (max_grid_size - (num_rows * 2.0)) / num_rows;
    
    max_tile_by_width.min(max_tile_by_height).min(40.0).max(15.0)
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
