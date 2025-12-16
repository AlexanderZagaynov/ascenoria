# Ascenoria Data Megapack (RON)

This folder contains game data for the open-source project **Ascenoria**,
a spiritual successor inspired by the classic 4X space strategy.

All text names and descriptions are newly generated; numeric parameters
(industry, research, costs, ranges, strength values, etc.) are derived
from reverse engineering and community analysis of the original game.

Common conventions:
- Every gameplay entity has:
  - `id` — stable machine identifier
  - `name_en` — English display name
- All RON files contain a single struct with a list of entities.

Files:
- `surface_cell_types.ron` — types of planet surface cells (e.g., White, Black)
- `surface_buildings.ron`  — buildings constructible on the surface
- `technologies.ron`       — researchable technologies
- `victory_conditions.ron` — victory conditions
- `scenarios.ron`          — game scenarios
