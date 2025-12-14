mod main_menu;
mod planet_screen;
mod shared;

use bevy::prelude::*;

use main_menu::MainMenuPlugin;
use planet_screen::PlanetScreenPlugin;
use shared::AppState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins((MainMenuPlugin, PlanetScreenPlugin))
        .add_systems(Update, esc_to_menu.run_if(in_state(AppState::Planet)))
        .run();
}

fn esc_to_menu(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}
