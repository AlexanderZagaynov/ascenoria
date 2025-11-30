//! Main menu screen implementation inspired by classic Ascendancy.
//!
//! Displays a title and menu buttons for game actions like New Game, Load, Save, and Exit.

use bevy::{ecs::hierarchy::ChildSpawnerCommands, ecs::message::MessageWriter, prelude::*};

/// Plugin that manages the main menu screen.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(
                Update,
                (button_system, menu_action_system).run_if(in_state(GameState::MainMenu)),
            );
    }
}

/// Game state machine for managing screens.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    SpeciesSelection,
    SpeciesIntro,
    InGame,
    StarSystem,
    Settings,
}

/// Marker component for all main menu UI entities.
#[derive(Component)]
struct MainMenuRoot;

/// Marker for menu buttons with their action type.
#[derive(Component, Clone, Copy)]
enum MenuButton {
    NewGame,
    LoadGame,
    SaveGame,
    Settings,
    Exit,
}

/// Colors for the menu UI - inspired by Ascendancy's color scheme.
mod colors {
    use bevy::prelude::*;

    /// Dark navy blue for button backgrounds.
    pub const BUTTON_NORMAL: Color = Color::srgb(0.08, 0.12, 0.20);
    /// Slightly lighter blue for hover state.
    pub const BUTTON_HOVERED: Color = Color::srgb(0.12, 0.18, 0.28);
    /// Even lighter for pressed state.
    pub const BUTTON_PRESSED: Color = Color::srgb(0.16, 0.24, 0.36);
    /// Teal/cyan border color.
    pub const BUTTON_BORDER: Color = Color::srgb(0.2, 0.5, 0.6);
    /// Light cyan text.
    pub const BUTTON_TEXT: Color = Color::srgb(0.7, 0.85, 0.9);
    /// Warm orange/amber background.
    pub const BACKGROUND: Color = Color::srgb(0.85, 0.55, 0.25);
    /// Darker orange for contrast areas.
    pub const BACKGROUND_DARK: Color = Color::srgb(0.45, 0.25, 0.12);
    /// Title text color - warm gold.
    pub const TITLE_TEXT: Color = Color::srgb(0.95, 0.75, 0.35);
    /// Subtitle/version text.
    pub const SUBTITLE_TEXT: Color = Color::srgb(0.7, 0.5, 0.25);
}

fn setup_main_menu(mut commands: Commands) {
    // Camera for the menu
    commands.spawn((Camera2d::default(), MainMenuRoot));

    // Root container - full screen with gradient-like background
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(colors::BACKGROUND),
            MainMenuRoot,
        ))
        .with_children(|parent| {
            // Title section
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                })
                .with_children(|title_section| {
                    // Main title
                    title_section.spawn((
                        Text::new("ASCENORIA"),
                        TextFont {
                            font_size: 96.0,
                            ..default()
                        },
                        TextColor(colors::TITLE_TEXT),
                    ));

                    // Subtitle
                    title_section.spawn((
                        Text::new("A 4X Space Strategy Game"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(colors::SUBTITLE_TEXT),
                    ));
                });

            // Menu container - dark panel with border
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(30.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(colors::BACKGROUND_DARK.with_alpha(0.9)),
                    BorderColor::all(colors::BUTTON_BORDER),
                ))
                .with_children(|menu| {
                    spawn_menu_button(menu, "New Game", MenuButton::NewGame, None);
                    spawn_menu_button(menu, "Load Game", MenuButton::LoadGame, Some("Alt-L"));
                    spawn_menu_button(menu, "Save Game", MenuButton::SaveGame, Some("Alt-S"));
                    spawn_menu_button(menu, "Settings", MenuButton::Settings, None);
                    spawn_menu_button(menu, "Exit", MenuButton::Exit, Some("Alt-X"));
                });

            // Version info at bottom
            parent.spawn((
                Text::new("v0.1.0 - Early Development"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::SUBTITLE_TEXT),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    ..default()
                },
            ));
        });
}

fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: MenuButton,
    shortcut: Option<&str>,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(350.0),
                height: Val::Px(55.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(colors::BUTTON_NORMAL),
            BorderColor::all(colors::BUTTON_BORDER),
            action,
        ))
        .with_children(|button| {
            // Main button text
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(colors::BUTTON_TEXT),
            ));

            // Shortcut text if provided
            if let Some(key) = shortcut {
                button.spawn((
                    Text::new(key),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::BUTTON_TEXT.with_alpha(0.6)),
                ));
            }
        });
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handles button interaction visual feedback.
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(colors::BUTTON_PRESSED);
                *border_color = BorderColor::all(colors::BUTTON_TEXT);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(colors::BUTTON_HOVERED);
                *border_color = BorderColor::all(colors::BUTTON_TEXT.with_alpha(0.8));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(colors::BUTTON_NORMAL);
                *border_color = BorderColor::all(colors::BUTTON_BORDER);
            }
        }
    }
}

/// Handles menu button actions.
fn menu_action_system(
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit_events: MessageWriter<AppExit>,
) {
    // Keyboard shortcuts
    let alt_pressed = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);

    if alt_pressed {
        if keyboard.just_pressed(KeyCode::KeyL) {
            info!("Load Game (keyboard shortcut)");
            // TODO: Implement load game
        } else if keyboard.just_pressed(KeyCode::KeyS) {
            info!("Save Game (keyboard shortcut)");
            // TODO: Implement save game
        } else if keyboard.just_pressed(KeyCode::KeyX) {
            exit_events.write(AppExit::Success);
        }
    }

    // Button clicks
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::NewGame => {
                    info!("Opening species selection...");
                    next_state.set(GameState::SpeciesSelection);
                }
                MenuButton::LoadGame => {
                    info!("Load Game clicked");
                    // TODO: Implement load game dialog
                }
                MenuButton::SaveGame => {
                    info!("Save Game clicked");
                    // TODO: Implement save game dialog
                }
                MenuButton::Settings => {
                    info!("Settings clicked");
                    next_state.set(GameState::Settings);
                }
                MenuButton::Exit => {
                    exit_events.write(AppExit::Success);
                }
            }
        }
    }
}
