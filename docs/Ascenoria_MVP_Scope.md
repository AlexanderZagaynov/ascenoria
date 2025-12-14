# Ascenoria — MVP Scope & Checklist

## 0. MVP Definition

**MVP = a complete, playable loop**, where the player can:

- Start a new game
- Manage an empire (at a minimal but meaningful level)
- Make decisions
- See consequences
- Reach a victory condition
- Exit to menu without soft-locks

The MVP does **not** require:
- final balance,
- final art,
- large content variety.

---

## 1. Galaxy & Planets

### 1.1 Galaxy Generation
- [ ] Seed-based galaxy generation
- [ ] Limited number of star systems
- [ ] Each system has 1–3 planets
- [ ] Deterministic results for the same seed

### 1.2 Planet Types (3)
For each planet type:
- [ ] ID
- [ ] Name (EN)
- [ ] Description (EN)
- [ ] Base parameters (population, productivity)
- [ ] Building constraints (if any)
- [ ] UI representation (icon / color / label)

---

## 2. Species (1)

### 2.1 Player Species
- [ ] ID
- [ ] Name (EN)
- [ ] Description (EN)
- [ ] Base modifiers:
  - [ ] Economy
  - [ ] Research
  - [ ] Industry
- [ ] Starting conditions:
  - [ ] Home planet
  - [ ] Starting technologies

---

## 3. Economy

### 3.1 Core Resources
- [ ] Population
- [ ] Food
- [ ] Industry
- [ ] Research

### 3.2 Rules
- [ ] Population growth rule
- [ ] Food consumption
- [ ] Industry production
- [ ] Research generation

---

## 4. Buildings (5)

For each building:
- [ ] ID
- [ ] Gameplay role
- [ ] Build cost (Industry)
- [ ] Upkeep (if any)
- [ ] Effect
- [ ] Requirements (planet / tech)
- [ ] Name (EN)
- [ ] Description (EN)
- [ ] Icon (placeholder allowed)
- [ ] UI integration
- [ ] Effect is applied correctly

---

## 5. Technologies (10)

For each technology:
- [ ] ID
- [ ] Name (EN)
- [ ] Description (EN)
- [ ] Research cost
- [ ] Prerequisites
- [ ] Effect:
  - unlocks content OR
  - removes a restriction
- [ ] Research UI integration
- [ ] Can be completed

> For MVP, every technology must unlock something tangible.

---

## 6. Ships

### 6.1 Hulls (2)
For each hull:
- [ ] ID
- [ ] Size / class
- [ ] Slot count
- [ ] Module restrictions
- [ ] Build cost
- [ ] Name (EN)
- [ ] Description (EN)

### 6.2 Modules (3)
Minimum set:
- Weapon
- Engine
- Shield or Special

For each module:
- [ ] ID
- [ ] Type
- [ ] Effect
- [ ] Cost
- [ ] Requirements
- [ ] Name (EN)
- [ ] Description (EN)

---

## 8. Victory (1)

### 8.1 Victory Condition
- [ ] Clearly defined rule
- [ ] Checked each turn
- [ ] Victory screen
- [ ] Return to menu

---

## 9. UI (Minimum Required)

- [ ] Main menu
- [ ] Galaxy view
- [ ] Planet view
- [ ] Build menu
- [ ] Research screen
- [ ] Ship screen
- [ ] Victory screen

---

## 10. Text & Localization

- [ ] All entities have Name.EN
- [ ] All entities have Description.EN
- [ ] No placeholders like TBD in final MVP

---

## 11. Technical Minimum

- [ ] Data loading from tables
- [ ] Data validation on startup
- [ ] No crashes on invalid data
- [ ] Runs on:
  - [ ] Linux
  - [ ] Windows
  - [ ] macOS (best effort)

---

## 12. MVP Definition of Done

The MVP is complete if:

- [ ] A full game can be played from start to victory
- [ ] All MVP entities are present
- [ ] No soft-locks
- [ ] All texts are filled in English
- [ ] All required UI actions are possible
- [ ] The game exits cleanly
