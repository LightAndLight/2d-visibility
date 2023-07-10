# Exercise 4

Hide entities that are in shadow.

Builds on [exercise 3](../exercise-3/index.md).
Uses the "shadow map" to calculate whether an entity is visible to the player character.

## Issues

* All of the issues from [exercise 1](../exercise-1/index.md#issues).
* When a shadow partially intersects an NPC, the NPC appears on top of the shadow. (Inherited from
  [exercise 3](../exercise-3/index.md))

  I tried to fix this with Z-ordering: putting the shadows on a higher "layer" than all the things
  that shadows can occlude, but lead to the shadows *also* occluding the entities that create
  them.
  This is because I cast shadows from all sides of the occluder, including sides close to the
  player character.
  When these close-to-the-player-character sides cast a shadow, the shadow goes *through* the
  occluding object.
  For the Z-order trick to work, only the sides that lead to "outward emanating" shadows should
  generate shadow objects.
  This would also be a performance improvement, because each occluder would generate fewer shadows.

* Equates "point is lit" with "point is in player character's line of sight".

  This system behaves as if there is a single light source in the same position as the player
  character.
  When this is the case, whether or not a point is lit corresponds to whether it's in the player's
  line of sight.
  This doesn't hold for static light sources.
  A static light source illuminates its surroundings regardless of what the player character can
  see.
  The player character's line of sight determines which points/entities should be rendered.

## References

Point-in-polygon testing:

* <http://www.faqs.org/faqs/graphics/algorithms-faq/>: "How do I find if a point lies within a polygon?"
* [Even-odd rule on Wikipedia](https://en.m.wikipedia.org/wiki/Even%E2%80%93odd_rule)