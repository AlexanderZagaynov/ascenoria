//! Species introduction screen.
//!
//! Displays an atmospheric introduction to the selected species before starting
//! the game, showing their portrait, name, mission briefing, and lore.

use bevy::prelude::*;

use crate::data::{GameData, HasDescription, Language, NamedEntity};
use crate::main_menu::GameState;
use crate::species_selection::NewGameSettings;

/// Plugin for the species introduction screen.
pub struct SpeciesIntroPlugin;

impl Plugin for SpeciesIntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SpeciesIntro), setup_intro_screen)
            .add_systems(OnExit(GameState::SpeciesIntro), cleanup_intro_screen)
            .add_systems(
                Update,
                (continue_system, keyboard_navigation_system)
                    .run_if(in_state(GameState::SpeciesIntro)),
            );
    }
}

/// Color constants for the intro screen.
mod colors {
    use bevy::prelude::*;

    /// Deep space background.
    pub const BACKGROUND: Color = Color::srgb(0.02, 0.02, 0.05);
    /// Panel background with transparency.
    pub const PANEL_BG: Color = Color::srgba(0.1, 0.1, 0.15, 0.85);
    /// Panel border color.
    pub const PANEL_BORDER: Color = Color::srgb(0.3, 0.4, 0.5);
    /// Title color (golden).
    pub const TITLE: Color = Color::srgb(0.9, 0.8, 0.4);
    /// Subtitle color.
    pub const SUBTITLE: Color = Color::srgb(0.7, 0.7, 0.8);
    /// Main text color.
    pub const TEXT: Color = Color::srgb(0.8, 0.8, 0.85);
    /// Muted text for hints.
    pub const HINT_TEXT: Color = Color::srgb(0.5, 0.5, 0.6);
    /// Portrait placeholder background.
    pub const PORTRAIT_BG: Color = Color::srgb(0.15, 0.15, 0.2);
    /// Portrait border.
    pub const PORTRAIT_BORDER: Color = Color::srgb(0.4, 0.5, 0.6);
    /// Button normal.
    pub const BUTTON_NORMAL: Color = Color::srgb(0.2, 0.25, 0.3);
    /// Button hover.
    pub const BUTTON_HOVER: Color = Color::srgb(0.3, 0.35, 0.4);
    /// Button pressed.
    pub const BUTTON_PRESSED: Color = Color::srgb(0.15, 0.2, 0.25);
}

/// Marker component for all intro screen UI entities.
#[derive(Component)]
struct IntroScreenRoot;

/// Marker for the continue button.
#[derive(Component)]
struct ContinueButton;

/// Marker for the back button.
#[derive(Component)]
struct BackButton;

/// Marker for star background entities.
#[derive(Component)]
struct StarBackground;

/// Sets up the species introduction screen.
fn setup_intro_screen(
    mut commands: Commands,
    settings: Res<NewGameSettings>,
    game_data: Option<Res<GameData>>,
) {
    // Get selected species info
    let (species_name, species_description, species_id) = game_data
        .as_ref()
        .and_then(|data| data.species().get(settings.selected_species_index))
        .map(|s| {
            (
                s.name(Language::En).to_string(),
                s.description(Language::En).to_string(),
                s.id.clone(),
            )
        })
        .unwrap_or_else(|| {
            (
                "Unknown Species".to_string(),
                "No description available.".to_string(),
                "unknown".to_string(),
            )
        });

    // Generate mission briefing text based on species
    let mission_briefing = generate_mission_briefing(&species_name, &species_id);

    // Spawn star background
    spawn_star_background(&mut commands);

    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND),
            IntroScreenRoot,
        ))
        .with_children(|parent| {
            // Main content panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(1000.0),
                        height: Val::Px(700.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(30.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(colors::PANEL_BG),
                    BorderColor::all(colors::PANEL_BORDER),
                    BorderRadius::all(Val::Px(12.0)),
                ))
                .with_children(|panel| {
                    // Title section
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::bottom(Val::Px(10.0)),
                            border: UiRect::bottom(Val::Px(1.0)),
                            ..default()
                        })
                        .with_children(|title_area| {
                            // Species name (large title)
                            title_area.spawn((
                                Text::new(species_name.clone()),
                                TextFont {
                                    font_size: 48.0,
                                    ..default()
                                },
                                TextColor(colors::TITLE),
                            ));

                            // Subtitle
                            title_area.spawn((
                                Text::new("Supreme Galactic Contenders"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(colors::SUBTITLE),
                                Node {
                                    margin: UiRect::top(Val::Px(5.0)),
                                    ..default()
                                },
                            ));
                        });

                    // Content area (portrait + text)
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(30.0),
                            margin: UiRect::top(Val::Px(20.0)),
                            ..default()
                        })
                        .with_children(|content| {
                            // Left side - Portrait
                            content
                                .spawn((
                                    Node {
                                        width: Val::Px(300.0),
                                        height: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect::all(Val::Px(20.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(colors::PORTRAIT_BG),
                                    BorderColor::all(colors::PORTRAIT_BORDER),
                                    BorderRadius::all(Val::Px(8.0)),
                                ))
                                .with_children(|portrait_area| {
                                    // Portrait placeholder
                                    portrait_area
                                        .spawn((
                                            Node {
                                                width: Val::Px(240.0),
                                                height: Val::Px(320.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                border: UiRect::all(Val::Px(1.0)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.1, 0.12, 0.15)),
                                            BorderColor::all(colors::PANEL_BORDER),
                                            BorderRadius::all(Val::Px(4.0)),
                                        ))
                                        .with_children(|placeholder| {
                                            // Placeholder text
                                            placeholder.spawn((
                                                Text::new("[Species Portrait]"),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(colors::HINT_TEXT),
                                            ));
                                        });

                                    // "Leader" label below portrait
                                    portrait_area.spawn((
                                        Text::new("Species Leader"),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(colors::SUBTITLE),
                                        Node {
                                            margin: UiRect::top(Val::Px(15.0)),
                                            ..default()
                                        },
                                    ));
                                });

                            // Right side - Text content
                            content
                                .spawn(Node {
                                    flex_grow: 1.0,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(20.0),
                                    overflow: Overflow::scroll_y(),
                                    ..default()
                                })
                                .with_children(|text_area| {
                                    // Mission briefing section
                                    text_area
                                        .spawn(Node {
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        })
                                        .with_children(|section| {
                                            section.spawn((
                                                Text::new("MISSION BRIEFING"),
                                                TextFont {
                                                    font_size: 18.0,
                                                    ..default()
                                                },
                                                TextColor(colors::TITLE),
                                            ));

                                            section.spawn((
                                                Text::new(mission_briefing.clone()),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(colors::TEXT),
                                                Node {
                                                    margin: UiRect::top(Val::Px(10.0)),
                                                    ..default()
                                                },
                                            ));
                                        });

                                    // Species lore section
                                    text_area
                                        .spawn(Node {
                                            flex_direction: FlexDirection::Column,
                                            margin: UiRect::top(Val::Px(10.0)),
                                            ..default()
                                        })
                                        .with_children(|section| {
                                            section.spawn((
                                                Text::new("SPECIES PROFILE"),
                                                TextFont {
                                                    font_size: 18.0,
                                                    ..default()
                                                },
                                                TextColor(colors::TITLE),
                                            ));

                                            section.spawn((
                                                Text::new(species_description.clone()),
                                                TextFont {
                                                    font_size: 16.0,
                                                    ..default()
                                                },
                                                TextColor(colors::TEXT),
                                                Node {
                                                    margin: UiRect::top(Val::Px(10.0)),
                                                    ..default()
                                                },
                                            ));
                                        });
                                });
                        });

                    // Bottom buttons
                    panel
                        .spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: UiRect::top(Val::Px(20.0)),
                            ..default()
                        })
                        .with_children(|button_area| {
                            // Back button
                            button_area
                                .spawn((
                                    Button,
                                    Node {
                                        padding: UiRect::new(
                                            Val::Px(30.0),
                                            Val::Px(30.0),
                                            Val::Px(15.0),
                                            Val::Px(15.0),
                                        ),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(colors::BUTTON_NORMAL),
                                    BorderColor::all(colors::PANEL_BORDER),
                                    BorderRadius::all(Val::Px(6.0)),
                                    BackButton,
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("← Back to Selection"),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(colors::TEXT),
                                    ));
                                });

                            // Continue button
                            button_area
                                .spawn((
                                    Button,
                                    Node {
                                        padding: UiRect::new(
                                            Val::Px(30.0),
                                            Val::Px(30.0),
                                            Val::Px(15.0),
                                            Val::Px(15.0),
                                        ),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(colors::BUTTON_NORMAL),
                                    BorderColor::all(colors::PANEL_BORDER),
                                    BorderRadius::all(Val::Px(6.0)),
                                    ContinueButton,
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("Begin Your Journey →"),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(colors::TEXT),
                                    ));
                                });
                        });
                });

            // Hint text at bottom
            parent.spawn((
                Text::new("Press ENTER to continue or ESC to go back"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::HINT_TEXT),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

/// Generates a mission briefing based on the species.
fn generate_mission_briefing(species_name: &str, species_id: &str) -> String {
    // Mission briefing varies by species personality
    match species_id {
        "minions" => format!(
            "The {} have emerged from their homeworld with one supreme purpose: galactic domination through any means necessary. \
            Your cunning and resourcefulness will be tested as you navigate the treacherous waters of interstellar politics.\n\n\
            Build your empire. Crush your enemies. The galaxy awaits its new masters.",
            species_name
        ),
        "chamachies" => format!(
            "The noble {} embark upon their greatest journey yet. Guided by ancient traditions and unwavering honor, \
            your people seek to bring enlightenment to the cosmos.\n\n\
            Forge alliances with worthy species. Expand your influence through wisdom and strength. The stars call to your destiny.",
            species_name
        ),
        "orfa" => format!(
            "The mysterious {} venture forth from the depths of their ocean world. \
            Your unique perspective and adaptability will prove invaluable in the harsh vacuum of space.\n\n\
            Explore strange new worlds. Discover the secrets of the cosmos. Your journey into the unknown begins now.",
            species_name
        ),
        "govorom" => format!(
            "The ancient {} have awakened after millennia of contemplation. \
            With vast knowledge accumulated over eons, your species now turns its gaze to the stars.\n\n\
            Share your wisdom or guard it jealously. The choice is yours. The universe trembles at your awakening.",
            species_name
        ),
        "saurians" => format!(
            "The mighty {} march forth to claim their rightful place among the stars. \
            Proud warriors and fierce competitors, your people know only victory.\n\n\
            Conquer. Dominate. Rule. The weak shall bow before the strong.",
            species_name
        ),
        "arbryls" => format!(
            "The enigmatic {} spread their tendrils across the galaxy. \
            Patient and methodical, your species plays the long game in the cosmic struggle for supremacy.\n\n\
            Grow. Adapt. Consume. The galaxy is but fertile ground for your kind.",
            species_name
        ),
        "frutmaka" => format!(
            "The industrious {} set forth to build a new future among the stars. \
            Masters of technology and engineering, no challenge is too great for your ingenuity.\n\n\
            Create. Innovate. Construct. The galaxy shall know your works.",
            species_name
        ),
        "shevar" => format!(
            "The ethereal {} drift through the cosmos seeking harmony and balance. \
            Your connection to the fundamental forces of the universe grants unique insight.\n\n\
            Seek balance. Find harmony. Bring peace to a chaotic galaxy... or not.",
            species_name
        ),
        "dubtaks" => format!(
            "The resilient {} emerge from their harsh homeworld ready for anything. \
            Survivors by nature, your people thrive where others would perish.\n\n\
            Endure. Persist. Overcome. The galaxy's challenges are nothing compared to home.",
            species_name
        ),
        _ => format!(
            "The {} embark on an epic journey to the stars. \
            A new chapter in your species' history begins today.\n\n\
            Explore the cosmos. Build your civilization. \
            Write your legend among the stars.",
            species_name
        ),
    }
}

/// Spawns decorative star background.
fn spawn_star_background(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            StarBackground,
            IntroScreenRoot,
            ZIndex(-1),
        ))
        .with_children(|parent| {
            // Spawn some decorative "stars" as small nodes
            let star_positions = [
                (10.0, 15.0, 2.0),
                (25.0, 8.0, 3.0),
                (40.0, 22.0, 2.0),
                (55.0, 5.0, 2.5),
                (70.0, 18.0, 2.0),
                (85.0, 12.0, 3.0),
                (15.0, 75.0, 2.0),
                (30.0, 85.0, 2.5),
                (50.0, 78.0, 2.0),
                (65.0, 90.0, 3.0),
                (80.0, 82.0, 2.0),
                (92.0, 70.0, 2.5),
                (5.0, 45.0, 2.0),
                (95.0, 40.0, 2.0),
                (88.0, 55.0, 2.5),
                (8.0, 60.0, 2.0),
            ];

            for (x, y, size) in star_positions {
                parent.spawn((
                    Node {
                        width: Val::Px(size),
                        height: Val::Px(size),
                        position_type: PositionType::Absolute,
                        left: Val::Percent(x),
                        top: Val::Percent(y),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.8, 0.8, 0.9, 0.6)),
                    BorderRadius::all(Val::Px(size / 2.0)),
                ));
            }
        });
}

/// Cleans up intro screen entities.
fn cleanup_intro_screen(mut commands: Commands, query: Query<Entity, With<IntroScreenRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handles button clicks for continue and back.
fn continue_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&ContinueButton>,
            Option<&BackButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color, is_continue, is_back) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(colors::BUTTON_PRESSED);
                if is_continue.is_some() {
                    info!("Continuing to galaxy...");
                    next_state.set(GameState::InGame);
                } else if is_back.is_some() {
                    info!("Returning to species selection...");
                    next_state.set(GameState::SpeciesSelection);
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(colors::BUTTON_HOVER);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(colors::BUTTON_NORMAL);
            }
        }
    }
}

/// Handles keyboard navigation (Enter to continue, Escape to go back).
fn keyboard_navigation_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        info!("Returning to species selection...");
        next_state.set(GameState::SpeciesSelection);
    } else if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        info!("Continuing to galaxy...");
        next_state.set(GameState::InGame);
    }
}
