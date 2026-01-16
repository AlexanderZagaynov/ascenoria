Feature: Planet view connectivity logic
  As a player
  I want buildings to power adjacent tiles from the base
  So that construction rules feel consistent and predictable

  Scenario: Base connects itself and its orthogonal neighbors
    Given a 3 by 3 surface with a base and a passage north of it
    When I update the connectivity state
    Then the base and its orthogonal neighbors are connected
    And the diagonal corner is connected via the passage
