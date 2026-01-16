Feature: Planet surface generation
  As a player
  I want planets to be generated deterministically from a seed
  So that saved games and tests can reproduce worlds

  Scenario: Generate a planet from a deterministic seed
    Given a deterministic planet seed
    Then the planet surface is a 10 by 10 grid
    And the base is placed on a white tile
