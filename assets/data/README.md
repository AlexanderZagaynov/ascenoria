# Ascenoria Data Megapack (TOML)

This folder contains game data for the open-source project **Ascenoria**,
a spiritual successor inspired by the classic 4X space strategy.

All text names and descriptions are newly generated; numeric parameters
(industry, research, costs, ranges, strength values, etc.) are derived
from reverse engineering and community analysis of the original game.

Common conventions:
- Every gameplay entity has:
  - `id` — stable machine identifier
  - `name_en` — English display name
- All TOML files are flat lists of `[[...]]` tables.

Files:
- `surface_cell_types.toml` — types of planet surface cells (e.g., White, Black)
- `surface_buildings.toml`  — buildings constructible on the surface
- `technologies.toml`       — researchable technologies
- `victory_conditions.toml` — victory conditions
- `scenarios.toml`          — game scenarios
