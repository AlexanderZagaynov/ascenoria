@manual
Feature: Main menu presentation and navigation
  As a player
  I want a clear landing screen with game actions
  So that I can start or exit the game confidently

  Background:
    Given the game launches to the main menu

  Scenario: Main menu layout and copy
    Then the title "ASCENORIA" is visible
    And the subtitle "A 4X Space Strategy Game" is visible
    And the version label is visible at the bottom of the screen

  Scenario: New Game button flow
    When I click the "New Game" button
    Then the game transitions to the planet view screen

  Scenario: Exit button and keyboard shortcut
    When I click the "Exit" button
    Then the application exits successfully
    When I press Alt+X on the keyboard
    Then the application exits successfully
