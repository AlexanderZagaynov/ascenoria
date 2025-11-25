# Ascenoria Data Megapack (TOML)

This folder contains game data for the open-source project **Ascenoria**,
a spiritual successor inspired by the classic 4X space strategy.

All text names and descriptions are newly generated; numeric parameters
(industry, research, costs, ranges, strength values, etc.) are derived
from reverse engineering and community analysis of the original game.

Common conventions:
- Every gameplay entity has:
  - `id` — stable machine identifier
  - `name = { en = "...", ru = "..." }`
  - `description = { en = "...", ru = "..." }`
- All TOML files are flat lists of `[[...]]` tables.

Files:
- `species.toml`              — playable species
- `planet_sizes.toml`         — planet sizes
- `planet_surfaces.toml`      — planet surface classifications
- `planetary_buildings.toml`  — surface buildings and improvements
- `planetary_satellites.toml` — orbital structures
- `planetary_projects.toml`   — long-running planetary projects
- `ship_hulls.toml`           — ship hull size classes
- `ships_engines.toml`        — ship engine modules
- `ships_weapons.toml`        — ship weapon modules
- `ships_shields.toml`        — defensive shields
- `ships_scanners.toml`       — sensor / scanner modules
- `ships_special.toml`        — special ship modules
- `research.toml`             — research technologies (cost only, no tree yet)
- `victory_conditions.toml`   — abstract victory conditions (design-level)
