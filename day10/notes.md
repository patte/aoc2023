| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this 

| [north, south]
- [east, west] 
L [north, east]
J [north, west]
7 [south, west]
F [south, east]
. []
S [north, south, east, west]

.....
.S-7.
.|.|.
.L-J.
.....

step_count = 0
pos = [1, 1]
[1,0] => go east, needs west
[0,-1]=> go south, needs north
[-1,0]=> go west, needs east
[0,1] => go north, needs south

answer = step_count / 2


-L|F7
7S-7|
L|7||
-L-J|
L|-JF