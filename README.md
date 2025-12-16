# Ascenoria

Ascenoria is a Rust/Bevy strategy prototype that loads most game content from TOML data files. Core assets live under `assets/data`, and additional content can be layered on top through lightweight mods.

## Modding quickstart

- Place each mod under `assets/mods/<mod_id>/data/`.
- Supported files mirror the base data set: `surface_cell_types.ron`, `surface_buildings.ron`, `technologies.ron`, `victory_conditions.ron`, and `scenarios.ron`.
- You can add new entries or override existing ones by `id`. When multiple mods define the same `id`, the one loaded last wins.
- Load order is deterministic: mods are sorted by `priority` (higher values load later) and then by folder name.
- Optional `mod.ron` in the mod folder can set `priority`:

  ```ron
  (
      priority: 10,
  )
  ```

- Schema compatibility is tracked via `data_schema_version` (default `1`) in `assets/data/manifest.ron`. Mods can also declare a `data_schema_version` alongside `priority` in `mod.ron`; versions newer than the runtime will be rejected.
- Technology prerequisites from `research_prereqs.ron` are merged by `(from, to)` pair with the same last-wins rule.

## Data linting


- The tool reuses the game loader checks and warns about missing localizations or ids that are not snake_case.
