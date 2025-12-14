# Ascenoria — MVP

## Project

**Ascenoria** is an open-source strategy game inspired by *Ascendancy*, built with **Rust + Bevy**.

The goal is to hard-minimize the project into a **fully playable MVP vertical slice** with a closed gameplay loop:

**build → generate resources → research → win**

This document defines the **authoritative MVP scope** and instructions for Codex to clean the codebase and implement only what is required.

---

## MVP Scope

### World

- The entire game world consists of **a single planet**
- There are no other navigable layers, maps, or views
- The planet is shown directly on the **Planet Screen**
- The planet surface is an **isometric grid**
- The grid has exactly **five cell types**
- Construction rule:
  - A building can be constructed only **adjacent to an existing building**
  - **Exception**: the very first building may be placed **anywhere**

---

## Content

### Surface Buildings

Exactly five surface buildings exist:

- **Housing** — increases population capacity / living space
- **Food** — increases food production
- **Industry** — increases production
- **Science** — increases research
- **Connector**
  - Can be built on unusable (“black”) cells
  - Provides no yields
  - Counts as a building for adjacency purposes
  - Used to extend build chains across unusable terrain

### Planet Cells

- Exactly five cell types exist
- Three cell types are usable
- One cell type is unusable (“black”)
- Usable cells accept any building
- Unusable cells accept only the **Connector**

### Technologies

Exactly three technologies exist:

- Hull
- Engine
- Generator

### Victory Condition

- The player wins immediately after **all technologies are researched**

### Civilization

- Exactly one civilization
- No opponents or AI

### Localization

- English only

---

## UI Scope

Only two screens exist:

### Main Menu

- Start Game
- Quit

### Planet Screen

- Planet surface grid
- Resource counters (food, industry, science)
- Current research progress
- Technology list
- Building placement interface
- End Turn button
- Victory message when all technologies are researched

No other screens, overlays, or menus exist.

---

## Design and Data Principles

- Only data required for the MVP may exist
- No future-oriented abstractions

---

## Gameplay Loop

- Start the game from the Main Menu
- Enter the Planet Screen
- Place the first building anywhere
- Place subsequent buildings only adjacent to existing ones
- End the turn:
  - Buildings generate food, industry, and science
  - Science advances research
  - Industry advances construction
- Research all technologies
- Display Victory
- Return to Main Menu

---

## Mandatory Rules

### Turn System

- An **End Turn** button exists
- Each turn:
  - Buildings generate resources
  - Research progresses
  - Construction progresses or completes
- Formulas must be simple and documented

### Adjacency Rule

A cell is buildable if:

- The cell is empty
- And either:
  - No buildings exist yet, or
  - At least one orthogonal neighbor contains any building

### Connector Rules

- May be built only on unusable cells
- Has zero yields
- Counts as a building for adjacency checks

### Victory

- When all technologies are researched:
  - Display a Victory message
  - Allow returning to the Main Menu or allow to continue the game

---

## Quality Requirements

- The project must compile and run using `cargo run`
- The game must be playable end-to-end
- Unused crates and features must be removed
- Add lightweight validation or tests for:
  - Adjacency logic
  - Victory condition

