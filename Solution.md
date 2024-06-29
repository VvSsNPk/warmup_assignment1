# Solution Summary
## Problem Description
In this assignment we are tasked to do navigation of a wumpus cave with
a vaccum cleaner. There is a representation of the wumpus cave that is given
in a text file format like below 

````text
XXXXXXXXXXXXXXXXXX
XXXXXXX  XXXXXXXXX
XXX XX   XXXXXXXXX
XX          XXXXXX
X X     P     XXXX
X           S XXXX
XX     XX     XXXX
XX     XXX P  XXXX
XXX X XXX     XXXX
XXXXX     XXXXXXXX
XXXXX  XXXXXXXXXXX
XXXXXXXXXXXXXXXXXX
````
So above is the representation of the files given. Here 'X' means wall
empty space means the positions where the vacuum cleaner can move and 
P are teleportation Portals. There are only two portals in a given map.
### Problem Files Description:
There are different files given with different problems, I will give a short description
of the problems below
+ First type of problem is where a plan is given, and we need to check whether the plan is valid or not
+ Second is find plan problems where a map is given, and we need to find a plan
Here also there is a variation:
+ Some problem files have 2 portals as shown in the above diagram and if the cleaner moves into a portal it will teleport in another portal location
+ Another type of variant is the problem files where start is not given, and we need to assume that the cleaner can start in any position.
## Approaches Takes
Now here I will be going over my approaches taken for the problems.

### Check Plan:
for check plan problems it simple, I created a representation of the whole map and just checked the given plan
if the whole map is cleaned then I say the plan is good plan, if not I will just list out the uncleaned spots and say bad plan

### Find Plan:
This is a little bit challenging as we need to find a suitable plan for a given problem file,



## Conclusion