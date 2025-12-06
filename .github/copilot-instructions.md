# Copilot Instructions for Ascenoria

## Project Overview

Ascenoria is a data-driven 4X strategy game built with **Bevy 0.17.3** and **Rust Edition 2024**.
See `AGENTS.md` for the full agent workflow, milestone structure, and GitHub issue conventions.

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
All screens are variants of `GameState` in `src/main_menu.rs`:
```rust
pub enum GameState {
    MainMenu, SpeciesSelection, SpeciesIntro, InGame, StarSystem, PlanetView, Settings
}
```

### Modal Dialog Pattern
For overlay dialogs (see `src/galaxy_map.rs`):
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

All game content lives in **TOML files** under `assets/data/`.

- Load via `data::load_game_data("assets/data")` returning `(GameData, GameRegistry)`
- Each data type has a corresponding Rust struct in `src/data.rs`
- Use `GameRegistry` for ID-based lookups
- Computed stats are derived via `game_data.compute() -> GameDataComputed`

### TOML Structure Convention
```toml
[[entity]]
id = "unique_id"
name = { en = "English", ru = "Русский" }
description = { en = "...", ru = "..." }
```

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `src/main.rs` | App entry, plugin registration |
| `src/main_menu.rs` | `GameState` enum, main menu |
| `src/data.rs` | TOML loading, all data structs |
| `src/galaxy_map.rs` | Galaxy view, modal system pattern |
| `src/planet_view.rs` | Planet surface, building placement |
| `assets/data/README.md` | TOML file conventions |

---

## Build & Validate

```bash
cargo check           # Fast compilation check
cargo fmt             # Format code
cargo test            # Run unit tests
cargo run --bin data_lint -- assets/data   # Validate TOML data
```

---

## Color Scheme

Screens define colors in a local `mod colors` block (see examples in `main_menu.rs`, `galaxy_map.rs`).
Use Ascendancy-inspired palettes: navy blues, teals, warm oranges/golds.

---

## Common Traits

```rust
trait HasId { fn id(&self) -> &str; }
trait NamedEntity { fn name(&self, lang: Language) -> &str; }
trait HasDescription { fn description(&self, lang: Language) -> &str; }
```

All entities support `Language::En` and `Language::Ru` localization.
