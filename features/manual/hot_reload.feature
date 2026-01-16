@manual
Feature: Game data hot reload and mod folders
  As a developer or modder
  I want the game to watch base data and mods
  So that edits can be reloaded without restarting

  Scenario: Base data hot reload
    Given the game is running with asset watching enabled
    When I change a file under assets/data
    Then the game reloads the data pack without crashing
    And the updated data is reflected in new planet views

  Scenario: Mods folder detection
    Given a mods folder exists under assets/mods
    When the game starts
    Then the mods folder is watched for changes

  Scenario: Missing mods folder
    Given no mods folder exists under assets/mods
    When the game starts
    Then the game logs a message that mod loading is skipped
