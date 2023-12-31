# Exercise 2

<div align="center">

<!-- ../videos/exercise_2 -->
[exercise_2.webm](https://github.com/LightAndLight/2d-visibility/assets/2536121/267680fd-f721-4bce-ac8a-669f5bd7e144)

</div>

Skip rendering of structures that aren't visible to the player.

Builds on [exercise 1](../exercise-1/index.md).
Instead of changing the NPC's colour, it is marked "not visible" to the game engine when the player
character can't see it.

## Issues

* All the issues from exercise 1.

* No visual feedback for areas that are occluded.

  Areas of the screen that are visible to the player's character appear the same as screen areas
  that are occluded.
  When the NPC "comes into view" it appears on the screen for no apparent reason.
