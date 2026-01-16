@manual
Feature: Planet view UI and interaction
  As a player
  I want to interact with planet tiles and UI panels
  So that I can build, advance turns, and track progress

  Background:
    Given I am viewing a generated planet

  Scenario: Top bar controls
    Then the top bar shows a quit button and an end-turn button
    When I click the end-turn button
    Then the turn counter increments by one

  Scenario: Build menu modal
    When I click a connected empty tile
    Then the build menu modal opens centered on screen
    And the modal lists available buildings with costs
    When I choose a building from the modal
    Then the modal closes
    And the building is queued for production

  Scenario: Production queue panel
    Then the left panel lists queued production projects in FIFO order
    And each queue entry displays progress toward completion

  Scenario: Planet info modal
    When I open the planet info modal
    Then the modal displays planet name, population, and growth info
    And the modal can be dismissed via the OK button

  Scenario: Tile visuals update
    When a tile changes state (building placed or power connected)
    Then the corresponding tile mesh and material update in the 3D view
    And the hover cursor continues to track the current tile

  Scenario: Victory message
    When a victory condition is met
    Then a victory overlay appears with the victory text
