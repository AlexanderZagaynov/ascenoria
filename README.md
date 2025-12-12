# Ascenoria

Ascenoria is a Rust/Bevy strategy prototype that loads most game content from TOML data files. Core assets live under `assets/data`, and additional content can be layered on top through lightweight mods.

## Modding quickstart

- Place each mod under `assets/mods/<mod_id>/data/`.
- Supported files mirror the base data set: `species.toml`, `planet_sizes.toml`, `planet_surfaces.toml`, `planetary_buildings.toml`, `planetary_satellites.toml`, `planetary_projects.toml`, `ship_hulls.toml`, `ships_engines.toml`, `ships_weapons.toml`, `ships_shields.toml`, `ships_scanners.toml`, `ships_special.toml`, `research.toml`, `research_prereqs.toml`, `victory_conditions.toml`, and `victory_rules.toml`.
- You can add new entries or override existing ones by `id`. When multiple mods define the same `id`, the one loaded last wins.
- Load order is deterministic: mods are sorted by `priority` (higher values load later) and then by folder name.
- Optional `mod.toml` in the mod folder can set `priority`:

  ```toml
  priority = 10
  ```

- Schema compatibility is tracked via `data_schema_version` (default `1`) in `assets/data/manifest.toml`. Mods can also declare a `data_schema_version` alongside `priority` in `mod.toml`; versions newer than the runtime will be rejected.
- Technology prerequisites from `research_prereqs.toml` are merged by `(from, to)` pair with the same last-wins rule.

## Data linting


- The tool reuses the game loader checks and warns about missing localizations or ids that are not snake_case.
