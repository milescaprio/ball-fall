# ball-fall

A quick & fun little 2D physics engine in rust, using Piston. For Lincoln Programming Club.
Uses integration and unit conversion for precise & generic location. Collisions use conservation of momentum and energy on a rotated axis.

Unfinished. Possible additions in the future:
-Static forces; balls clip through each other when stuck in a corner/have too much momentum, lack of this implementation, and time granularity. Additionally, rolling balls bounce at very high increasing frequencies on the ground.
-Rotational motion: balls are currently treated as point particles
-Friction: sideways collisions of balls are unrealistic
-More efficient data structure: the current scan for collisions is O(n^2). By sectioning the coordinate plane, intersections can be more efficiently evaluated (not in time complexity, but in practical execution).
