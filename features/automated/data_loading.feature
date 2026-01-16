Feature: Game data loading and registry lookup
  As a player
  I want the base data pack to load reliably
  So that gameplay systems can reference validated IDs

  Scenario: Load the base dataset from assets/data
    Given the base game data directory
    When I load the game data
    Then the dataset includes surface cell types, buildings, technologies, victories, and scenarios
    And the registry can resolve key ids
