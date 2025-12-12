use bevy::prelude::*;
use crate::galaxy_map::colors;
use super::super::types::ModalIcon;

pub fn spawn_modal_top_section(panel: &mut ChildSpawnerCommands, icon: &ModalIcon) {
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
            let icon_text = match icon {
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
                // Icon container
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

                // Planet preview circle
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
}

pub fn spawn_modal_message_section(
    panel: &mut ChildSpawnerCommands,
    message: &str,
    detail: &Option<String>,
) {
    panel
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::new(Val::Px(25.0), Val::Px(25.0), Val::Px(20.0), Val::Px(10.0)),
            ..default()
        })
        .with_children(|msg_section| {
            msg_section.spawn((
                Text::new(message),
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

            if let Some(detail_text) = detail {
                msg_section.spawn((
                    Text::new(detail_text),
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
}
