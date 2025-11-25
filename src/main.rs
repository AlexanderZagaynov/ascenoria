mod data;

use bevy::{
    prelude::*,
    text::{TextColor, TextFont},
};

use data::{load_game_data, GameData, Language, LocalizedEntity};

/// Plugin that loads game data from TOML files and registers it as a resource.
pub struct GameDataPlugin {
    /// Path to the directory containing the canonical TOML files.
    pub data_path: String,
}

impl Default for GameDataPlugin {
    fn default() -> Self {
        Self {
            data_path: "assets/data".to_string(),
        }
    }
}

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        match load_game_data(&self.data_path) {
            Ok((game_data, registry)) => {
                info!("Loaded game data from {}", self.data_path);
                let computed = game_data.compute();
                app.insert_resource(registry);
                app.insert_resource(computed);
                app.insert_resource(game_data);
            }
            Err(err) => {
                panic!("Failed to load game data from {}: {}", self.data_path, err);
            }
        }
    }
}

/// Current language selection for UI rendering.
#[derive(Resource, Default)]
struct LocalizationSettings {
    language: Language,
}

impl LocalizationSettings {
    fn toggle(&mut self) {
        self.language = match self.language {
            Language::En => Language::Ru,
            Language::Ru => Language::En,
        };
    }
}

#[derive(Component)]
struct LocalizedPreviewText;

fn localized_preview(game_data: &GameData, language: Language) -> String {
    let mut lines = Vec::new();

    if let Some(species) = game_data.species().first() {
        lines.push(species.name(language).to_string());
        lines.push(species.description(language).to_string());
    }

    if let Some(hull) = game_data.hull_classes().first() {
        lines.push(hull.name(language).to_string());
        lines.push(hull.description(language).to_string());
    }

    if let Some(engine) = game_data.engines().first() {
        lines.push(engine.name(language).to_string());
        lines.push(engine.description(language).to_string());
    }

    lines.join("\n")
}

fn setup_ui(
    mut commands: Commands,
    game_data: Res<GameData>,
    localization: Res<LocalizationSettings>,
) {
    let preview = localized_preview(&game_data, localization.language);
    commands.spawn((
        Text::new(preview),
        TextFont {
            font_size: 18.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        LocalizedPreviewText,
    ));
}

fn toggle_language(
    input: Res<ButtonInput<KeyCode>>,
    mut localization: ResMut<LocalizationSettings>,
    game_data: Res<GameData>,
    mut text_query: Query<&mut Text, With<LocalizedPreviewText>>,
) {
    if !input.just_pressed(KeyCode::KeyL) {
        return;
    }

    localization.toggle();
    let preview = localized_preview(&game_data, localization.language);

    for mut text in &mut text_query {
        text.0 = preview.clone();
    }
}

fn main() {
    App::new()
        .init_resource::<LocalizationSettings>()
        .add_plugins((DefaultPlugins, GameDataPlugin::default()))
        .add_systems(Startup, setup_ui)
        .add_systems(Update, toggle_language)
        .run();
}
