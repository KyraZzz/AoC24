# Day 6
The time has come -- the annual Advent of Code programming challenge. This year, I plan to tackle the challenge using the Rust programming language. I see it as a fantastic opportunity to deepen my understanding of idiomatic Rust practices.

I'll document my journey to share with the community, hoping it serves as a helpful resource for programmers who want to learn Rust in a fun and engaging way.

**Here is the ["Master" post](https://www.reddit.com/r/learnrust/comments/1h6vsbq/aoc2024_rust_tutorials_the_rusty_way_to_christmas/) if you want to check out solutions for other days.**

# Part 1
# Problem statement

Today's challenge involves navigating a grid with the following rules.

A guard, represented as `^` (initially facing up), moves according to these rules:
* Continue forward in the direction of the arrow until encountering an obstacle (`#`).
* Upon hitting an obstacle, turn 90 degrees to the right.

The task is to simulate the guard's movement and count the distinct positions visited before the guard exits the grid. In the example below, the guard visits 41 distinct positions.
```
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
```

# Solution skeleton
1. Initialize the grid by reading the input file into a 2D grid.
2. Locate the starting position of the guard (`^`), replacing it with `.` to avoid bugs when revisiting the starting position during simulation.
3. Write a simulation function, `out_of_grid()`, to calculate the number of distinct positions visited by the guard before exiting the grid.

```rust
fn part1() -> u32 {
    let mut m = init_map();
    let start = find_start(&m);
    m[start.0 as usize][start.1 as usize] = '.';

    out_of_grid(&(start.0, start.1, 0), &m).unwrap() as u32
}
```

# Initialise the grid
Parse the input file to construct a 2D grid:
```rust
fn init_map() -> Vec<Vec<char>> {
    let f = std::fs::File::open(FILE_PATH).unwrap();
    let r = BufReader::new(f);
    r.lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
```

# Find the starting position
We can use either a for loop or an iterator. While an iterator might be slightly more efficient for small datasets due to compiler optimizations, the performance difference is negligible in this case. 

(But as enthusiasts of functional programming, we’re all about the fun, right? So, let’s go with the iterator version! :D)

```rust
// For loop version
fn find_start(m: &[Vec<char>]) -> (i32, i32) {
    let mut start = (0, 0);
    for (i, row) in m.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '^' {
                start = (i as i32, j as i32);
            }
        }
    }
    start
}
// Iterator version
fn find_start(m: &[Vec<char>]) -> (i32, i32) {
    let start = m
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '^')
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .collect::<Vec<(i32, i32)>>()[0];
    start
}
```

# Simulate the game
Now the fun part - game simulation! 

Simulate the guard’s movement using a HashSet to track visited positions. The guard's movement directions are defined in anticlockwise order (up, right, down, left). A `(cur_idx + 1) % 4` operation is used to rotate 90 degrees to the right.

```rust
fn bound_check(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < height && y >= 0 && y < width
}
fn out_of_grid(start: &(i32, i32, usize), m: &[Vec<char>]) -> usize {
    let width = m[0].len();
    let height = m.len();
    // directions in an anticlockwise way: up, right, down, left
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut cur_x, mut cur_y, mut cur_idx) = start;
    let mut hits = HashSet::new();
    loop {
        hits.insert((cur_x, cur_y));

        let cur_step = dirs[cur_idx];
        let (new_x, new_y) = (cur_x + cur_step.0, cur_y + cur_step.1);
        if !bound_check(new_x, new_y, width as i32, height as i32) {
            return hits.len();
        }
        if m[new_x as usize][new_y as usize] == '#' {
            cur_idx = (cur_idx + 1) % 4;
        } else {
            (cur_x, cur_y) = (new_x, new_y);
        }
    }
}
```

# Final program
```rust
fn bound_check(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < height && y >= 0 && y < width
}

fn init_map() -> Vec<Vec<char>> {
    let f = std::fs::File::open(FILE_PATH).unwrap();
    let r = BufReader::new(f);
    r.lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_start(m: &[Vec<char>]) -> (i32, i32) {
    let start = m
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '^')
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .collect::<Vec<(i32, i32)>>()[0];
    start
}

fn out_of_grid(start: &(i32, i32, usize), m: &[Vec<char>]) -> usize {
    let width = m[0].len();
    let height = m.len();
    // directions in an anticlockwise way: up, right, down, left
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut cur_x, mut cur_y, mut cur_idx) = start;
    let mut hits = HashSet::new();
    loop {
        hits.insert((cur_x, cur_y));

        let cur_step = dirs[cur_idx];
        let (new_x, new_y) = (cur_x + cur_step.0, cur_y + cur_step.1);
        if !bound_check(new_x, new_y, width as i32, height as i32) {
            return hits.len();
        }
        if m[new_x as usize][new_y as usize] == '#' {
            cur_idx = (cur_idx + 1) % 4;
        } else {
            (cur_x, cur_y) = (new_x, new_y);
        }
    }
}

fn part1() -> u32 {
    let mut m = init_map();
    let start = find_start(&m);
    m[start.0 as usize][start.1 as usize] = '.';

    out_of_grid(&(start.0, start.1, 0), &m) as u32
}
```

# Part 2
# Problem statement

In Part 2, the goal is to detect loops in the guard's path:
* You can place one obstacle (`#`) on any `.` position (excluding the guard’s starting position).
* If the obstacle causes the guard to never exit the grid, it is counted as a success.

In the following simple example, there are six positions we can place an obstacle which causes the guard to move in a loop, I have denoted them as `O`.

```
....#.....
.........#
..........
..#.......
.......#..
..........
.#.O^.....
......OO#.
#O.O......
......#O..
```

# Brute-force solution is too slow
We can slightly modify the `out_of_bound()` function to detect loops. By iterating through the entire grid, we can change one `.` position at a time (excluding the guard's starting position) into an obstacle `#` and then check if this modification causes the guard to enter a loop (i.e., the guard never exits the grid).

Here is the updated `out_of_bound2()` function. It returns `None` if a loop is detected and `Some(i32)` if the guard successfully exits the grid.

To detect loops, we track whether the guard visits the same position **from the same direction** more than once. This means the `HashSet` must store not only the position but also the direction.

```rust
fn out_of_grid2(start: &(i32, i32, usize), m: &[Vec<char>]) -> Option<i32> {
    let width = m[0].len();
    let height = m.len();
    // directions in an anticlockwise way: up, right, down, left
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut cur_x, mut cur_y, mut cur_idx) = start;
    let mut hits = HashSet::new();
    loop {
        if hits.contains(&(cur_x, cur_y, cur_idx)) {
            return None;
        }
        hits.insert((cur_x, cur_y, cur_idx));

        let cur_step = dirs[cur_idx];
        let (new_x, new_y) = (cur_x + cur_step.0, cur_y + cur_step.1);
        if !bound_check(new_x, new_y, width as i32, height as i32) {
            return Some(
                hits.iter()
                    .map(|(i, j, _)| (*i, *j))
                    .collect::<HashSet<(i32, i32)>>().len() as i32,
            );
        }
        if m[new_x as usize][new_y as usize] == '#' {
            cur_idx = (cur_idx + 1) % 4;
        } else {
            (cur_x, cur_y) = (new_x, new_y);
        }
    }
}
```

If we attempt to iterate through the entire grid, modifying each `.` position (excluding the guard's starting position) one at a time, the process becomes excessively slow and inefficient. To improve the algorithm's performance, we can explore several optimisation strategies.

# Optmisation 1: Restrict Search to the Guard’s Route
Positions outside the guard's original route have no impact on its movements. Thus, instead of iterating over all grid positions, we can limit the search to only those positions visited by the guard in Part 1.

# Optmisation 2: Avoid Recalculating the Route
If a new obstacle is placed along the guard’s route, only the portion of the route after the obstacle is affected. Therefore, we can skip recalculating the path before the obstacle, focusing solely on the changes introduced by the obstacle placement.

# Optimisation 3: Use a 2D Boolean Grid Instead of a HashSet
Although a HashSet provides `O(1)` complexity for operations like insertions, deletions, and lookups, using a 2D boolean grid can be more efficient for this type of grid-based problem:

* Memory Efficiency: For smaller datasets, a 2D boolean array typically requires less memory than a HashSet.
* Cache Optimization: A 2D array provides a contiguous memory access pattern, which can result in better cache performance compared to the scattered access pattern of a HashSet.
* Reduced Overhead: Unlike a HashSet, accessing a 2D array does not involve hash computations, making it faster for small-scale grids.

# Ideas for further optimisations
An additional suggestion involves using a jump table to cache the positions and directions of obstacles. This would allow the guard to "fast-forward" directly to the next obstacle, significantly reducing computational overhead. (We leave this as an exercise for readers! :)

# Final program with 3 optimisation
```rust
fn out_of_grid2(start: &(i32, i32, usize), m: &[Vec<char>]) -> Option<i32> {
    let width = m[0].len();
    let height = m.len();
    // directions in an anticlockwise way: up, right, down, left
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut cur_x, mut cur_y, mut cur_idx) = start;
    // optimisation 3: using a 2D array rather than a hashset
    let mut hits = vec![vec![[false; 4]; m[0].len()]; m.len()];
    loop {
        if hits[cur_x as usize][cur_y as usize][cur_idx] {
            return None;
        }
        hits[cur_x as usize][cur_y as usize][cur_idx] = true;

        let cur_step = dirs[cur_idx];
        let (new_x, new_y) = (cur_x + cur_step.0, cur_y + cur_step.1);
        if !bound_check(new_x, new_y, width as i32, height as i32) {
            return Some(
                (hits
                    .iter()
                    .enumerate()
                    .flat_map(|(i, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|&(_, col)| col.iter().any(|&b| b))
                            .map(move |(j, _)| (i, j))
                    })
                    .collect::<Vec<(usize, usize)>>()
                    .len()) as i32,
            );
        }
        if m[new_x as usize][new_y as usize] == '#' {
            cur_idx = (cur_idx + 1) % 4;
        } else {
            (cur_x, cur_y) = (new_x, new_y);
        }
    }
}

fn part2() -> u32 {
    let mut m = init_map();
    let start = find_start(&m);
    m[start.0 as usize][start.1 as usize] = '.';

    let width = m[0].len();
    let height = m.len();

    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut cur_idx = 0;
    let (mut cur_x, mut cur_y) = (start.0, start.1);

    // optimisation 3: use 2D array instead of a hashset
    let mut visited = vec![vec![false; width]; height];
    let mut res = 0;
    loop {
        visited[cur_x as usize][cur_y as usize] = true;
        let cur_step = dirs[cur_idx];
        let (new_x, new_y) = (cur_x + cur_step.0, cur_y + cur_step.1);
        if !bound_check(new_x, new_y, width as i32, height as i32) {
            break;
        }
        if m[new_x as usize][new_y as usize] == '#' {
            cur_idx = (cur_idx + 1) % 4;
        } else {
            if (new_x, new_y) != start && !visited[new_x as usize][new_y as usize] {
                m[new_x as usize][new_y as usize] = '#';
                if out_of_grid2(&(cur_x, cur_y, cur_idx), &m).is_none() {
                    res += 1;
                }
                m[new_x as usize][new_y as usize] = '.';
            }
            (cur_x, cur_y) = (new_x, new_y);
        }
    }
    res as u32
}
```