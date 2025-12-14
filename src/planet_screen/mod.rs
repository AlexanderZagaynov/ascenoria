use std::collections::VecDeque;

use ascenoria::mvp::{
    BUILD_COST_INDUSTRY, Building, BuildingKind, CellType, GridPos, ResearchState, Resources,
    apply_turn_production, can_place_building, spend_industry_on_building,
};
use bevy::{
    ecs::message::{MessageReader, MessageWriter},
    prelude::*,
    window::PrimaryWindow,
};

use crate::shared::AppState;

pub struct PlanetScreenPlugin;

impl Plugin for PlanetScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EndTurnRequested>()
            .init_resource::<PlanetConfig>()
            .init_resource::<PlanetState>()
            .add_systems(OnEnter(AppState::Planet), setup_planet_screen)
            .add_systems(OnExit(AppState::Planet), cleanup_planet_screen)
            .add_systems(
                Update,
                (
                    tile_click_system,
                    ui_button_interactions,
                    ui_update_system,
                    end_turn_system,
                    update_tile_visuals,
                )
                    .run_if(in_state(AppState::Planet)),
            );
    }
}

#[derive(Message, Default)]
struct EndTurnRequested;

#[derive(Resource)]
struct PlanetConfig {
    width: i32,
    height: i32,
    tile_w: f32,
    tile_h: f32,
    origin: Vec2,
}

impl Default for PlanetConfig {
    fn default() -> Self {
        Self {
            width: 10,
            height: 10,
            tile_w: 72.0,
            tile_h: 36.0,
            origin: Vec2::new(220.0, -60.0),
        }
    }
}

#[derive(Resource)]
struct PlanetState {
    resources: Resources,
    research: ResearchState,
    selected_building: BuildingKind,
    victory: bool,
    turn: i32,
    construction_active: Option<GridPos>,
    construction_queue: VecDeque<GridPos>,
}

impl Default for PlanetState {
    fn default() -> Self {
        Self {
            resources: Resources::default(),
            research: ResearchState::default(),
            selected_building: BuildingKind::Housing,
            victory: false,
            turn: 1,
            construction_active: None,
            construction_queue: VecDeque::new(),
        }
    }
}

#[derive(Component)]
struct PlanetRoot;

#[derive(Component, Clone, Copy)]
struct TilePos(GridPos);

#[derive(Component, Clone, Copy)]
struct TileCell(CellType);

#[derive(Component, Clone, Copy)]
struct TileBuilding(Option<Building>);

#[derive(Component)]
struct TileSprite;

#[derive(Component)]
struct BuildingMarker;

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct ResourceText;

#[derive(Component)]
struct ResearchText;

#[derive(Component)]
struct TechListText;

#[derive(Component)]
struct TurnText;

#[derive(Component)]
struct SelectedBuildingText;

#[derive(Component)]
struct VictoryOverlay;

#[derive(Component, Clone, Copy)]
struct SelectBuildingButton(BuildingKind);

#[derive(Component)]
struct EndTurnButton;

#[derive(Component)]
struct ReturnToMenuButton;

#[derive(Component)]
struct ContinueButton;

fn setup_planet_screen(
    mut commands: Commands,
    config: Res<PlanetConfig>,
    mut planet: ResMut<PlanetState>,
) {
    *planet = PlanetState::default();
    commands.spawn((Camera2d::default(), PlanetRoot));

    for y in 0..config.height {
        for x in 0..config.width {
            let pos = GridPos { x, y };
            let cell_type = initial_cell_type(pos);
            commands.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(config.tile_w, config.tile_h)),
                    color: cell_color(cell_type),
                    ..default()
                },
                Transform::from_translation(grid_to_world(config.as_ref(), pos))
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
                TileSprite,
                TilePos(pos),
                TileCell(cell_type),
                TileBuilding(None),
                PlanetRoot,
            ));
        }
    }

    spawn_planet_ui(&mut commands);
}

fn cleanup_planet_screen(mut commands: Commands, query: Query<Entity, With<PlanetRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn initial_cell_type(pos: GridPos) -> CellType {
    if (pos.x == 4 && pos.y >= 2 && pos.y <= 7) || (pos.y == 6 && pos.x >= 1 && pos.x <= 4) {
        CellType::Void
    } else if (pos.x + pos.y) % 11 == 0 {
        CellType::Hills
    } else if (pos.x + pos.y) % 7 == 0 {
        CellType::Forest
    } else if (pos.x + 2 * pos.y) % 9 == 0 {
        CellType::Desert
    } else {
        CellType::Plains
    }
}

fn cell_color(cell_type: CellType) -> Color {
    match cell_type {
        CellType::Plains => Color::srgb(0.2, 0.55, 0.25),
        CellType::Forest => Color::srgb(0.12, 0.42, 0.18),
        CellType::Hills => Color::srgb(0.35, 0.45, 0.25),
        CellType::Desert => Color::srgb(0.65, 0.58, 0.25),
        CellType::Void => Color::srgb(0.05, 0.05, 0.06),
    }
}

fn building_color(kind: BuildingKind) -> Color {
    match kind {
        BuildingKind::Housing => Color::srgb(0.85, 0.85, 0.9),
        BuildingKind::Food => Color::srgb(0.3, 0.8, 0.3),
        BuildingKind::Industry => Color::srgb(0.85, 0.6, 0.25),
        BuildingKind::Science => Color::srgb(0.35, 0.65, 0.95),
        BuildingKind::Connector => Color::srgb(0.65, 0.65, 0.7),
    }
}

fn grid_to_world(config: &PlanetConfig, pos: GridPos) -> Vec3 {
    let x = (pos.x - pos.y) as f32 * (config.tile_w / 2.0);
    let y = (pos.x + pos.y) as f32 * (config.tile_h / 2.0);
    Vec3::new(
        x + config.origin.x,
        y + config.origin.y,
        pos.y as f32 * 0.01,
    )
}

fn world_to_grid(config: &PlanetConfig, world: Vec2) -> Option<GridPos> {
    let world = world - config.origin;

    let half_w = config.tile_w / 2.0;
    let half_h = config.tile_h / 2.0;

    if half_w <= 0.0 || half_h <= 0.0 {
        return None;
    }

    let fx = (world.x / half_w + world.y / half_h) / 2.0;
    let fy = (world.y / half_h - world.x / half_w) / 2.0;

    let base = GridPos {
        x: fx.round() as i32,
        y: fy.round() as i32,
    };

    for candidate in [
        base,
        GridPos {
            x: base.x - 1,
            y: base.y,
        },
        GridPos {
            x: base.x + 1,
            y: base.y,
        },
        GridPos {
            x: base.x,
            y: base.y - 1,
        },
        GridPos {
            x: base.x,
            y: base.y + 1,
        },
    ] {
        let center = Vec2::new(
            (candidate.x - candidate.y) as f32 * half_w,
            (candidate.x + candidate.y) as f32 * half_h,
        );
        let local = world - center;
        if local.x.abs() / half_w + local.y.abs() / half_h <= 1.0 {
            return Some(candidate);
        }
    }

    None
}

fn in_bounds(config: &PlanetConfig, pos: GridPos) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < config.width && pos.y < config.height
}

fn tile_click_system(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    config: Res<PlanetConfig>,
    mut planet: ResMut<PlanetState>,
    mut tiles: ParamSet<(
        Query<(&TilePos, &TileCell, &TileBuilding)>,
        Query<(&TilePos, &TileCell, &mut TileBuilding)>,
    )>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };
    let Ok(world) = camera.viewport_to_world_2d(camera_transform, cursor) else {
        return;
    };
    let Some(pos) = world_to_grid(&config, world) else {
        return;
    };
    if !in_bounds(&config, pos) {
        return;
    }

    let any_buildings_exist = tiles
        .p0()
        .iter()
        .any(|(_, _, building)| building.0.is_some());
    let has_neighbor = tiles.p0().iter().any(|(tile_pos, _, building)| {
        if building.0.is_none() {
            return false;
        }
        tile_pos.0.neighbors_4().into_iter().any(|n| n == pos)
    });

    for (tile_pos, tile_cell, mut tile_building) in &mut tiles.p1() {
        if tile_pos.0 != pos {
            continue;
        }

        let cell_is_empty = tile_building.0.is_none();
        let can_place = can_place_building(
            planet.selected_building,
            tile_cell.0,
            cell_is_empty,
            any_buildings_exist,
            has_neighbor,
        );
        if can_place.is_err() {
            return;
        }

        tile_building.0 = Some(Building {
            kind: planet.selected_building,
            remaining_industry: BUILD_COST_INDUSTRY,
        });

        if planet.construction_active.is_none() {
            planet.construction_active = Some(pos);
        } else {
            planet.construction_queue.push_back(pos);
        }
    }
}

fn update_tile_visuals(
    mut commands: Commands,
    config: Res<PlanetConfig>,
    mut tiles: Query<
        (
            Entity,
            &TilePos,
            &TileCell,
            &TileBuilding,
            Option<&Children>,
        ),
        Changed<TileBuilding>,
    >,
) {
    for (entity, tile_pos, tile_cell, tile_building, children) in &mut tiles {
        if let Some(children) = children {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        commands.entity(entity).insert((
            Sprite {
                custom_size: Some(Vec2::new(config.tile_w, config.tile_h)),
                color: cell_color(tile_cell.0),
                ..default()
            },
            Transform::from_translation(grid_to_world(config.as_ref(), tile_pos.0))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
        ));

        let Some(building) = tile_building.0 else {
            continue;
        };

        let alpha = if building.is_constructed() { 1.0 } else { 0.4 };

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(18.0, 18.0)),
                    color: building_color(building.kind).with_alpha(alpha),
                    ..default()
                },
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                BuildingMarker,
                PlanetRoot,
            ));
        });
    }
}

fn spawn_planet_ui(commands: &mut Commands) {
    let root = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            UiRoot,
            PlanetRoot,
        ))
        .id();

    commands.entity(root).with_children(|ui| {
        ui.spawn((
            Node {
                width: Val::Px(280.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.085, 0.1).with_alpha(0.95)),
            PlanetRoot,
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("Planet"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.95, 0.97)),
                PlanetRoot,
            ));

            panel.spawn((
                Text::new("Turn: 1"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.85)),
                TurnText,
                PlanetRoot,
            ));

            panel.spawn((
                Text::new("Food: 0 | Industry: 0 | Science: 0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.85)),
                ResourceText,
                PlanetRoot,
            ));

            panel.spawn((
                Text::new("Selected building: Housing"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.95)),
                SelectedBuildingText,
                PlanetRoot,
            ));

            panel
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(6.0),
                        ..default()
                    },
                    PlanetRoot,
                ))
                .with_children(|buttons| {
                    spawn_build_button(buttons, "Housing", BuildingKind::Housing);
                    spawn_build_button(buttons, "Food", BuildingKind::Food);
                    spawn_build_button(buttons, "Industry", BuildingKind::Industry);
                    spawn_build_button(buttons, "Science", BuildingKind::Science);
                    spawn_build_button(buttons, "Connector", BuildingKind::Connector);
                });

            panel.spawn((
                Text::new("Research: Hull (0/10)"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.85)),
                ResearchText,
                PlanetRoot,
            ));

            panel.spawn((
                Text::new("Techs: [ ] Hull  [ ] Engine  [ ] Generator"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.75, 0.75, 0.8)),
                TechListText,
                PlanetRoot,
            ));

            panel
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(220.0),
                        height: Val::Px(44.0),
                        margin: UiRect::top(Val::Px(8.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.35, 0.22)),
                    EndTurnButton,
                    PlanetRoot,
                ))
                .with_children(|b| {
                    b.spawn((
                        Text::new("End Turn"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.95, 0.97)),
                        PlanetRoot,
                    ));
                });
        });

        ui.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.65)),
            Visibility::Hidden,
            VictoryOverlay,
            PlanetRoot,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: Val::Px(420.0),
                        padding: UiRect::all(Val::Px(18.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.09, 0.1, 0.12)),
                    BorderColor::all(Color::srgb(0.25, 0.28, 0.33)),
                    PlanetRoot,
                ))
                .with_children(|modal| {
                    modal.spawn((
                        Text::new("Victory! All technologies researched."),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.95, 0.97)),
                        PlanetRoot,
                    ));
                    modal
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(260.0),
                                height: Val::Px(44.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.16, 0.3, 0.45)),
                            ReturnToMenuButton,
                            PlanetRoot,
                        ))
                        .with_children(|b| {
                            b.spawn((
                                Text::new("Return to Main Menu"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.95, 0.95, 0.97)),
                                PlanetRoot,
                            ));
                        });
                    modal
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(260.0),
                                height: Val::Px(44.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.22, 0.25)),
                            ContinueButton,
                            PlanetRoot,
                        ))
                        .with_children(|b| {
                            b.spawn((
                                Text::new("Continue"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.95, 0.95, 0.97)),
                                PlanetRoot,
                            ));
                        });
                });
        });
    });
}

fn spawn_build_button(parent: &mut ChildSpawnerCommands, label: &str, kind: BuildingKind) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(220.0),
                height: Val::Px(36.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.16, 0.16, 0.19)),
            SelectBuildingButton(kind),
            PlanetRoot,
        ))
        .with_children(|b| {
            b.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.95)),
                PlanetRoot,
            ));
        });
}

fn ui_button_interactions(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SelectBuildingButton>,
            Option<&EndTurnButton>,
            Option<&ReturnToMenuButton>,
            Option<&ContinueButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut planet: ResMut<PlanetState>,
    mut end_turn: MessageWriter<EndTurnRequested>,
    mut next_state: ResMut<NextState<AppState>>,
    mut overlay_visibility: Query<&mut Visibility, With<VictoryOverlay>>,
) {
    for (interaction, mut bg, select, end_turn_btn, return_btn, cont_btn) in &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb(0.26, 0.26, 0.3));
                if let Some(select) = select {
                    planet.selected_building = select.0;
                }
                if end_turn_btn.is_some() {
                    let _ = end_turn.write(EndTurnRequested);
                }
                if return_btn.is_some() {
                    next_state.set(AppState::MainMenu);
                }
                if cont_btn.is_some() {
                    if let Some(mut v) = overlay_visibility.iter_mut().next() {
                        *v = Visibility::Hidden;
                    }
                }
            }
            Interaction::Hovered => *bg = BackgroundColor(Color::srgb(0.2, 0.2, 0.24)),
            Interaction::None => {
                // Restore a reasonable default (not perfect per-button styling, but minimal).
                *bg = BackgroundColor(Color::srgb(0.16, 0.16, 0.19));
            }
        }
    }
}

fn end_turn_system(
    mut end_turn: MessageReader<EndTurnRequested>,
    mut planet: ResMut<PlanetState>,
    mut tiles: Query<(&TilePos, &mut TileBuilding)>,
    mut overlay_visibility: Query<&mut Visibility, With<VictoryOverlay>>,
) {
    if end_turn.is_empty() {
        return;
    }
    end_turn.clear();

    planet.turn += 1;

    let buildings_iter = tiles
        .iter()
        .filter_map(|(_, building)| building.0)
        .map(|b| b);
    apply_turn_production(&mut planet.resources, buildings_iter);

    let science = planet.resources.science;
    planet.resources.science = planet.research.advance_with_science(science);

    let mut available_industry = planet.resources.industry;
    while available_industry > 0 {
        let Some(active_pos) = planet.construction_active else {
            break;
        };

        let mut progressed = false;
        for (tile_pos, mut tile_building) in &mut tiles {
            if tile_pos.0 != active_pos {
                continue;
            }
            let Some(mut building) = tile_building.0 else {
                continue;
            };
            spend_industry_on_building(&mut building, &mut available_industry);
            progressed = true;
            tile_building.0 = Some(building);
            if building.is_constructed() {
                planet.construction_active = planet.construction_queue.pop_front();
            }
        }

        if !progressed {
            planet.construction_active = planet.construction_queue.pop_front();
        }
    }
    planet.resources.industry = available_industry;

    if planet.research.all_researched() {
        planet.victory = true;
        if let Some(mut v) = overlay_visibility.iter_mut().next() {
            *v = Visibility::Visible;
        }
    }
}

fn ui_update_system(
    planet: Res<PlanetState>,
    mut resource_text: Query<&mut Text, With<ResourceText>>,
    mut research_text: Query<&mut Text, With<ResearchText>>,
    mut tech_text: Query<&mut Text, With<TechListText>>,
    mut turn_text: Query<&mut Text, With<TurnText>>,
    mut selected_text: Query<&mut Text, With<SelectedBuildingText>>,
    mut overlay_visibility: Query<&mut Visibility, With<VictoryOverlay>>,
) {
    if planet.is_changed() {
        if let Some(mut t) = resource_text.iter_mut().next() {
            *t = Text::new(format!(
                "Food: {} | Industry: {} | Science: {} | Pop cap: {}",
                planet.resources.food,
                planet.resources.industry,
                planet.resources.science,
                planet.resources.pop_capacity
            ));
        }
        if let Some(mut t) = turn_text.iter_mut().next() {
            *t = Text::new(format!("Turn: {}", planet.turn));
        }
        if let Some(mut t) = selected_text.iter_mut().next() {
            *t = Text::new(format!(
                "Selected building: {}",
                building_label(planet.selected_building)
            ));
        }
        if let Some(mut t) = research_text.iter_mut().next() {
            let label = planet
                .research
                .current_tech()
                .map(tech_label)
                .unwrap_or("Complete");
            *t = Text::new(format!(
                "Research: {} ({}/{})",
                label, planet.research.progress, planet.research.cost_per_tech
            ));
        }
        if let Some(mut t) = tech_text.iter_mut().next() {
            *t = Text::new(format!(
                "Techs: [{}] Hull  [{}] Engine  [{}] Generator",
                if planet.research.completed[0] {
                    "x"
                } else {
                    " "
                },
                if planet.research.completed[1] {
                    "x"
                } else {
                    " "
                },
                if planet.research.completed[2] {
                    "x"
                } else {
                    " "
                },
            ));
        }
        if planet.victory {
            if let Some(mut v) = overlay_visibility.iter_mut().next() {
                *v = Visibility::Visible;
            }
        }
    }
}

fn building_label(kind: BuildingKind) -> &'static str {
    match kind {
        BuildingKind::Housing => "Housing",
        BuildingKind::Food => "Food",
        BuildingKind::Industry => "Industry",
        BuildingKind::Science => "Science",
        BuildingKind::Connector => "Connector",
    }
}

fn tech_label(tech: ascenoria::mvp::Tech) -> &'static str {
    match tech {
        ascenoria::mvp::Tech::Hull => "Hull",
        ascenoria::mvp::Tech::Engine => "Engine",
        ascenoria::mvp::Tech::Generator => "Generator",
    }
}
