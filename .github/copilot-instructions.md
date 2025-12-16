# Copilot Instructions for Ascenoria

## Project Overview

Ascenoria is a data-driven 4X strategy game built with **Bevy 0.17.3** and **Rust Edition 2024**.
See `AGENTS.md` for the full agent workflow, milestone structure, and GitHub issue conventions.

---

## Module Organization

The codebase follows a consistent naming convention:

| Pattern | Purpose | Examples |
|---------|---------|----------|
| `*_data/` | Data types, generation, game logic | `galaxy_data/`, `star_data/`, `planet_data/` |
| `*_view/` | UI screens, rendering, user interaction | `galaxy_view/`, `star_view/`, `planet_view/` |
| `data_types/` | RON structs, loaders, validation | entities, loaders, registry |
| `game_data/` | Bevy plugin for data initialization | hot_reload, initialization |

### Source Structure

```
src/
├── main.rs                 # App entry, plugin registration
├── lib.rs                  # Library exports
├── main_menu/              # Main menu screen
│
├── data_types/             # RON data structures
│   ├── entities/           # Surface, tech, victory, scenario
│   ├── loaders/            # RON loading
│   ├── registry/           # ID-based lookups
│   └── tests/              # Unit tests
│
├── game_data/              # Bevy resource initialization
│   ├── initialization.rs   # Load and insert resources
│   └── hot_reload.rs       # File watching for dev
│
├── planet_view/            # Planet surface screen
│   ├── setup/              # Scene setup
│   ├── rendering/          # Materials, mesh
│   ├── modal/              # Planet dialogs
│   └── ui/                 # Panels, top bar
│
└── planet_data/            # Planet generation logic
```

---

## Architecture Patterns

### Screen/Plugin Pattern
Each game screen is a standalone Bevy plugin with this structure:
```rust
pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenState>()
           .add_systems(OnEnter(GameState::Screen), setup_screen)
           .add_systems(OnExit(GameState::Screen), cleanup_screen)
           .add_systems(Update, (system1, system2).run_if(in_state(GameState::Screen)));
    }
}
```
- Use a `Root` marker component on all screen entities for cleanup
- Register in `src/main.rs` with `.add_plugins(ScreenPlugin)`

### GameState Enum
All screens are variants of `GameState` in `src/main_menu/mod.rs`:
```rust
pub enum GameState {
    MainMenu, SpeciesSelection, SpeciesIntro, InGame, StarSystem, PlanetView, Settings
}
```

### Modal Dialog Pattern
For overlay dialogs (see `src/galaxy_view/modal/`):
```rust
#[derive(Resource, Default)]
pub struct InfoModalState {
    pub visible: bool,
    pub message: String,
    pub buttons: Vec<ModalButton>,
}
```
- Toggle visibility via the resource
- Render conditionally in a system checking `state.visible`

---

## Bevy 0.17 API Notes

These patterns differ from older Bevy versions:

| Pattern | Correct Usage |
|---------|---------------|
| Border color | `BorderColor::all(color)` |
| Text justification | `TextLayout::new_with_justify(Justify::Center)` |
| Child spawning | Use closures with `ChildSpawnerCommands` inline |
| Hierarchy imports | `bevy::ecs::hierarchy::ChildSpawnerCommands` |

---

## Data Loading

All game content lives in **RON files** under `assets/data/`.

- Load via `data_types::load_game_data("assets/data")` returning `(GameData, GameRegistry)`
- Each data type has a corresponding Rust struct in `src/data_types/entities/`
- Use `GameRegistry` for ID-based lookups

### RON Structure Convention
```ron
(
    entity: [
        (
            id: "unique_id",
            name_en: "English",
        ),
    ],
)
```

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `src/main.rs` | App entry, plugin registration |
| `src/main_menu/mod.rs` | `GameState` enum, main menu plugin |
| `src/data_types/mod.rs` | Data types re-exports |
| `src/game_data/mod.rs` | Game data Bevy plugin |
| `src/planet_view/mod.rs` | Planet surface screen plugin |
| `assets/data/README.md` | RON file conventions |

---

## Build & Validate

```bash
cargo check           # Fast compilation check
cargo fmt             # Format code
cargo test            # Run unit tests
```

---

## Color Scheme

Screens define colors in a local `mod colors` block (see examples in `main_menu/colors.rs`, `galaxy_view/mod.rs`).
Use Ascendancy-inspired palettes: navy blues, teals, warm oranges/golds.

---

## Common Traits

```rust
trait HasId { fn id(&self) -> &str; }
trait NamedEntity { fn name(&self, lang: Language) -> &str; }
trait HasDescription { fn description(&self, lang: Language) -> &str; }
```

All entities support `Language::En` and `Language::Ru` localization.
