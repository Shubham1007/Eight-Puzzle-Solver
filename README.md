










# Eight-Puzzle-Solver:
A solver for the 8 puzzle in Rust using the A* search algorithm.

```
$ cargo run
Initial state:
6 2 8
7 1 4
0 3 5
Solving...
Solution: [Left, Down, Left, Up, Right, Down, Left, Down, Right, Up, Up, Left, Down, Down, Right, Up, Right, Down]
Path length: 18
6 2 8
7 1 4
0 3 5
Left
6 8 2
7 1 4
0 3 5
Down
6 1 2
7 8 4
0 3 5
Left
6 1 2
8 7 4
0 3 5
Up
8 1 2
6 7 4
0 3 5
Right
1 8 2
6 7 4
0 3 5
Down
1 7 2
6 8 4
0 3 5
Left
1 7 2
8 6 4
0 3 5
Down
1 7 2
0 6 4
8 3 5
Right
1 7 2
0 6 4
3 8 5
Up
1 7 2
0 8 4
3 6 5
Up
1 8 2
0 7 4
3 6 5
Left
8 1 2
0 7 4
3 6 5
Down
0 1 2
8 7 4
3 6 5
Down
0 1 2
3 7 4
8 6 5
Right
0 1 2
3 7 4
6 8 5
Up
0 1 2
3 8 4
6 7 5
Right
0 1 2
3 4 8
6 7 5
Down
0 1 2
3 4 5
6 7 8
```


