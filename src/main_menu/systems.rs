use bevy::{ecs::hierarchy::ChildSpawnerCommands, ecs::message::MessageWriter, prelude::*};
use super::colors;
use super::components::{MainMenuRoot, MenuButton};
use super::GameState;

pub fn setup_main_menu(mut commands: Commands) {
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

pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handles button interaction visual feedback.
pub fn button_system(
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
pub fn menu_action_system(
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
                    next_state.set(GameState::GameOptions);
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
