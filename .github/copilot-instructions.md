# Copilot Instructions for Ascenoria

## Project Overview

Ascenoria is a data-driven 4X strategy game built with **Bevy 0.17** and **Rust Edition 2024**.
For autonomous agent workflows and GitHub issue conventions, see `AGENTS.md`.

---

## Architecture: Data vs View Separation

The codebase enforces a strict separation between data/logic and presentation:

| Suffix | Purpose | Example |
|--------|---------|---------|
| `*_data/` | Generation, types, pure logic | `planet_data/` - `PlanetSurface`, `BuildingType`, generation |
| `*_view/` | Bevy systems, UI, rendering | `planet_view/` - 3D scene, input handling, UI panels |
| `data_types/` | RON structs + loaders | Entities loaded from `assets/data/*.ron` |

**Key insight**: `planet_data/types.rs` defines runtime game state (`BuildingType` enum), while `data_types/entities/surface.rs` defines data-file schemas (`SurfaceBuilding` struct loaded from RON).

---

## Screen Plugin Pattern

Each game screen is a standalone Bevy plugin (see `planet_view/mod.rs`):

```rust
impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlanetViewState>()
           .add_systems(OnEnter(GameState::PlanetView), setup::setup_planet_view)
           .add_systems(OnExit(GameState::PlanetView), systems::cleanup_planet_view)
           .add_systems(Update, (/* systems */).run_if(in_state(GameState::PlanetView)));
    }
}
```

- Use marker components (`PlanetViewRoot`, `PlanetView3D`) on entities for cleanup
- State enum lives in `main_menu/mod.rs`: `GameState { MainMenu, PlanetView }`

---

## Connectivity System (Core Game Mechanic)

Buildings must be "connected" to the Base via a chain of grid-extending structures.
Implementation in `planet_view/logic.rs` uses BFS:

1. Find Base building (start node)
2. BFS through buildings where `counts_for_adjacency: true` (from RON data)
3. Mark grid nodes AND their orthogonal neighbors as connected
4. Only connected tiles can receive new buildings

---

## Production Queue System

Buildings are constructed through a FIFO production queue managed by `PlanetViewState`.

### Data Structures (`planet_view/types.rs`)

```rust
pub struct ProductionProject {
    pub project_type: ProjectType,      // Building(BuildingType)
    pub total_cost: u32,                // From RON: production_cost
    pub progress: u32,                  // Accumulated production points
    pub target_tile_index: usize,       // Where building will be placed
}

pub struct PlanetViewState {
    pub production_queue: VecDeque<ProductionProject>,
    pub build_menu_open: bool,
    pub build_menu_target_tile: Option<usize>,
    // ... resource totals, surface, etc.
}
```

### Turn Processing Flow (`planet_view/systems.rs::end_turn`)

1. **Yield Calculation**: Sum `yields_*` from all buildings (data-driven via RON)
2. **Queue Processing**: Apply accumulated `production` to front project
3. **Completion Check**: If `progress >= total_cost`:
   - Pop project from queue
   - Place building on target tile
   - Trigger connectivity recalculation
4. **Research**: Accumulate science toward tech unlocks

### Build Menu Flow

```
Click connected tile → build_menu_open = true, target_tile = idx
                    → spawn_build_menu() shows options
Select building     → push ProductionProject to queue
                    → build_menu_open = false
```

---

## Data Loading Flow

```
assets/data/*.ron  →  load_game_data()  →  (GameData, GameRegistry)
                                              ↓
                      Bevy Resources:  game_data: Res<GameData>
                                       registry: Res<GameRegistry>
```

- `GameData`: Vectors of loaded entities
- `GameRegistry`: HashMap indices for O(1) ID lookups
- Building definitions in RON include `yields_*`, `production_cost`, `counts_for_adjacency`

---

## Bevy 0.17 API Notes

| Pattern | Correct Usage |
|---------|---------------|
| Border color | `BorderColor::all(color)` |
| Child spawning | Closures with `ChildSpawnerCommands` |
| Messages/Events | `app.add_message::<TileUpdateEvent>()` |
| Despawn | `.despawn()` (not `DespawnRecursiveExt`) |

---

## Build Commands

```bash
cargo check    # Fast compilation check (use this frequently)
cargo fmt      # Format before committing
cargo test     # Run unit tests
cargo run      # Launch game
```

---

## Key Files

| File | Purpose |
|------|---------|
| `src/planet_view/logic.rs` | Connectivity BFS algorithm |
| `src/planet_view/types.rs` | `PlanetViewState`, `ProductionProject` |
| `src/planet_data/types.rs` | `BuildingType`, `PlanetSurface`, `SurfaceTile` |
| `src/data_types/entities/surface.rs` | `SurfaceBuilding` RON schema |
| `assets/data/surface_buildings.ron` | Building definitions (yields, costs) |

---

## Color Conventions

Screens define colors in local `mod colors` blocks. Use Ascendancy-inspired palette:
- Panel backgrounds: dark navy/slate
- Accent colors: teal, gold, orange
- Building colors: defined per-building in RON data
