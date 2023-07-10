# Exercise 3

<div align="center">

<!-- ../videos/exercise_3.webm -->
[exercise_3.webm](https://github.com/LightAndLight/2d-visibility/assets/2536121/89c027b9-884b-4d48-9034-956d4137c790)

</div>

Apply "lighting" to areas that are visible to the player character.

Builds on [exercise 2](../exercise-2/index.md).
Adds a visual representation of which areas are visible to the player character.

## Issues

* All of the issues from [exercise 1](../exercise-1/index.md#issues).
* Light/shadow system is not connected to visibility checks.

  I have to scan all the occluders to build the "shadow map" - the areas that are hidden from the
  player character.
  A visibility check involving the player character will re-scan all the occluders.
  Why not use the "shadow map" to decide whether an entity is visible to the player character?
  I think this could lead to a better simulation. 

* When a shadow partially intersects an NPC, the NPC appears on top of the shadow.

* "Visible" entities in general (such as walls) aren't occluded by the shadows.

  A wall that is in shadow still appears on screen.

## References

* 2D lighting techniques:
  
  * <https://ncase.me/sight-and-light/>

    Great interactive demos that show how to break a problem like this down into smaller components.

  * <https://slembcke.github.io/SuperFastHardShadows>

    A more technical explanation of how to do this style of shadowing.

* [Quadrilateral vertex order](https://stackoverflow.com/a/69104076/2884502):

  > * Define a new point C called the centre; C might be anything, and the exact choice of C will affect the result, especially if the polygon defined by your points is not convex. A simple choice for C is to take the barycentre of your list of points (i.e., take the average of the coordinates in your list).
  > * For every point P in your list, calculate the oriented angle alpha_P between the x-axis and vector CP.
  > * Sort the list of points by their associated angle; in increasing order for counterclockwise, or in decreasing order for clockwise.
