use bevy::prelude::*;
use crate::galaxy_view::colors;
use crate::galaxy_view::types::PanelButton;

/// Spawn a panel section button.
pub fn spawn_panel_section(
    panel: &mut ChildSpawnerCommands,
    label: &str,
    button_type: PanelButton,
) {
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
