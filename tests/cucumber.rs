use std::path::PathBuf;

use ascenoria::data_types::{load_game_data, GameData, GameRegistry};
use ascenoria::planet_data::{generate_planet, BuildingType, PlanetSurface, TileColor};
use ascenoria::planet_view::logic::update_connectivity;
use cucumber::{given, then, when, World as _};

#[derive(Debug, Default, cucumber::World)]
struct AscenoriaWorld {
    data_path: PathBuf,
    game_data: Option<GameData>,
    registry: Option<GameRegistry>,
    surface: Option<PlanetSurface>,
}

impl AscenoriaWorld {
    fn ensure_game_data(&mut self) {
        if self.game_data.is_some() && self.registry.is_some() {
            return;
        }
        let (data, registry) =
            load_game_data(self.data_path.clone()).expect("load_game_data failed");
        self.game_data = Some(data);
        self.registry = Some(registry);
    }
}

#[given("the base game data directory")]
async fn given_base_game_data_directory(world: &mut AscenoriaWorld) {
    world.data_path = PathBuf::from("assets/data");
}

#[when("I load the game data")]
async fn when_load_game_data(world: &mut AscenoriaWorld) {
    let (data, registry) =
        load_game_data(world.data_path.clone()).expect("load_game_data failed");
    world.game_data = Some(data);
    world.registry = Some(registry);
}

#[then("the dataset includes surface cell types, buildings, technologies, victories, and scenarios")]
async fn then_dataset_has_core_collections(world: &mut AscenoriaWorld) {
    let data = world.game_data.as_ref().expect("game data not loaded");
    assert!(!data.surface_cell_types().is_empty());
    assert!(!data.surface_buildings().is_empty());
    assert!(!data.technologies().is_empty());
    assert!(!data.victory_conditions().is_empty());
    assert!(!data.scenarios().is_empty());
}

#[then("the registry can resolve key ids")]
async fn then_registry_resolves_ids(world: &mut AscenoriaWorld) {
    let data = world.game_data.as_ref().expect("game data not loaded");
    let registry = world.registry.as_ref().expect("registry not loaded");

    assert!(registry.surface_cell_type(data, "cell_white").is_some());
    assert!(registry.surface_building(data, "building_base").is_some());
    assert!(registry.technology(data, "tech_terraforming").is_some());
    assert!(
        registry
            .victory_condition(data, "victory_cover_planet")
            .is_some()
    );
    assert!(registry.scenario(data, "scenario_mvp").is_some());
}

#[given("a deterministic planet seed")]
async fn given_deterministic_planet_seed(world: &mut AscenoriaWorld) {
    world.data_path = PathBuf::from("assets/data");
    world.surface = Some(generate_planet(42));
}

#[then("the planet surface is a 10 by 10 grid")]
async fn then_planet_surface_is_10x10(world: &mut AscenoriaWorld) {
    let surface = world.surface.as_ref().expect("surface not generated");
    assert_eq!(surface.row_width, 10);
    assert_eq!(surface.height(), 10);
}

#[then("the base is placed on a white tile")]
async fn then_base_is_on_white_tile(world: &mut AscenoriaWorld) {
    let surface = world.surface.as_ref().expect("surface not generated");
    let base_tile = surface
        .tiles
        .iter()
        .find(|tile| matches!(tile.building, Some(BuildingType::Base)));
    let base_tile = base_tile.expect("base building not found");
    assert_eq!(base_tile.color, TileColor::White);
}

#[given("a 3 by 3 surface with a base and a passage north of it")]
async fn given_surface_with_base_and_passage(world: &mut AscenoriaWorld) {
    let mut surface = PlanetSurface::new(3, 3);
    for tile in surface.tiles.iter_mut() {
        tile.color = TileColor::White;
    }

    if let Some(tile) = surface.get_mut(1, 1) {
        tile.building = Some(BuildingType::Base);
    }
    if let Some(tile) = surface.get_mut(1, 0) {
        tile.building = Some(BuildingType::Passage);
    }

    world.surface = Some(surface);
    world.data_path = PathBuf::from("assets/data");
}

#[when("I update the connectivity state")]
async fn when_update_connectivity(world: &mut AscenoriaWorld) {
    world.ensure_game_data();
    let data = world.game_data.as_ref().expect("game data not loaded");
    let registry = world.registry.as_ref().expect("registry not loaded");
    let mut surface = world.surface.take().expect("surface not generated");
    update_connectivity(&mut surface, data, registry);
    world.surface = Some(surface);
}

#[then("the base and its orthogonal neighbors are connected")]
async fn then_base_neighbors_connected(world: &mut AscenoriaWorld) {
    let surface = world.surface.as_ref().expect("surface not generated");
    let base = surface.get(1, 1).expect("missing base tile");
    assert!(base.connected);

    let north = surface.get(1, 0).expect("missing north tile");
    let south = surface.get(1, 2).expect("missing south tile");
    let west = surface.get(0, 1).expect("missing west tile");
    let east = surface.get(2, 1).expect("missing east tile");

    assert!(north.connected);
    assert!(south.connected);
    assert!(west.connected);
    assert!(east.connected);
}

#[then("the diagonal corner is connected via the passage")]
async fn then_diagonal_corner_connected(world: &mut AscenoriaWorld) {
    let surface = world.surface.as_ref().expect("surface not generated");
    let corner = surface.get(0, 0).expect("missing corner tile");
    assert!(corner.connected);
}

#[tokio::main]
async fn main() {
    AscenoriaWorld::run("./features/automated").await;
}
