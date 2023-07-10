# Exercise 1

<div align="center">

<!-- ../videos/exercise_1.webm -->
[exercise_1.webm](https://github.com/LightAndLight/boxybox/assets/2536121/0027dcbf-522f-4b0f-8709-e64fdd587106)

</div>

Symmetric visibility test with occlusion via line segment intersections.

There is a player (a blue square), an occluding wall (a black quadrilateral), and a non-player
character (a grey square).
When the player and NPC can't see each other, the NPC is coloured grey.
When the player and NPC *can* see each other, the NPC is coloured green.

## Mechanics

An occluder, such as a wall, consists of four lines forming a quadrilateral.
To check whether an entity `a` can see entity `b`, a line segment is created starting at `a` and
ending at `b`.
The occluders are searched one by one, checking whether the `a->b` line segment intersects any
line segment defined by an occluder.
If there is an intersection, it means that the "line of sight" from `a` to `b` has been blocked by
at least one side of the occluder.

## Issues

* The cost of a visibility check is proportional to the number of occluders.

* Many "obviously" irrelevant occluders will be checked.

  For example, the occluders that are behind entity `a` cannot possibly impede `a`'s line of sight
  to `b`.

* Entities have a 360 degree field of vision.

  A more advanced system would keep track an entity's "look direction", and assign the entity a
  restricted field of view. This would make visibility asymmetric: if I'm looking at you and you're
  looking somewhere else, I can see you while you can't see me.

* Systems have to "know what to look for".

  The visibility check takes two entities as arguments.
  I can check whether `a` sees `b`, but it's harder to find out what `a` can see in general.
  I'd like to be able to do this so that an NPC's actions can be influenced by what they see.

* Line of sight can be obscured by very small objects.

  If there is a single pixel-sized occluder blocking the line segment between two entities that are
  otherwise visible to each other, this system will report that they can't see each other.
