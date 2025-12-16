use bevy::{ecs::hierarchy::ChildSpawnerCommands, prelude::*};

use crate::main_menu::colors;
use crate::main_menu::components::{MainMenuRoot, MenuButton};

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
