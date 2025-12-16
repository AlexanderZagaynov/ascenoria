# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Major Refactor**: Switched to a strict MVP data schema.
  - Removed `Species`, `Ship`, `PlanetSize`, `PlanetSurfaceType`, etc.
  - Added `SurfaceCellType`, `SurfaceBuilding`, `Technology`, `VictoryCondition`, `Scenario`.
  - Updated data loaders and registry to support the new schema.
  - Replaced all TOML data files with MVP-compliant versions.

### Added
- Shared `HasId`/`NamedEntity` helpers for game entities and updated UI helpers to use the generic accessors.
- CLI data linter to validate TOML packs and warn about id naming or missing localizations.
- Track data schema version via `manifest.toml`/`mod.toml` with migration hook for future TOML changes.
- Hot reload TOML game data via Bevy's asset change detection for faster iteration in development.
- Load additional data packs from `assets/mods`, with deterministic priority-based overrides by `id` and TOML parity with core assets.
- Victory rules (e.g., domination threshold) now load from `victory_rules.toml` and can be overridden by mods.
- Build a `GameRegistry` with typed identifier wrappers for every collection and reject duplicate IDs during validation.
- Compute derived weapon, engine, and planetary item stats during loading and expose them via read-only caches.
- Enforce strict asset validation for slot sizes, hull metrics, and shield values to align with loader rules.

[Unreleased]: https://github.com/AlexanderZagaynov/ascenoria/compare/HEAD...HEAD
