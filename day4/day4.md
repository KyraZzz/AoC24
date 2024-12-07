# Day 4
# Part 1
# Problem statement
You are given a grid containing characters from `['X', 'M', 'A', 'S']`. The task is to count the occurrences of the string `"XMAS"`. These occurrences can be horizontal, vertical, diagonal, reversed, and overlapping.

For example, the following grid contains 18 occurrences of the string `"XMAS"`:

```
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
```

The grid with only `"XMAS"` occurrences highlighted:

```
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
```

# Find all starting points
You can use a nested for loop to iterate over the grid and collect positions where `'X'` appears.

```rust
// using a for loop
fn part1() {
  ...
  let mut starts = vec![];
  let height = map.len() as i32;
  let width = map[0].len() as i32;
  for i in 0..height {
      for j in 0..width {
          if map[i as usize][j as usize] == 'X' {
              starts.push((i as usize, j as usize));
          }
      }
  }
  ...
}
```

Alternatively, using iterators makes the code concise and expressive, particularly for functional programming enthusiasts. Iterator may be more efficient for small datasets where compiler optimisations can be effective, although performance gain may not be super significant for most use cases. 

```rust
// using an iterator
fn part1() {
  ...
  let starts = map
      .iter()
      .enumerate()
      .flat_map(|(i, row)| {
          row.iter()
              .enumerate()
              .filter(|&(_, &c)| c == 'X')
              .map(move |(j, _)| (i, j))
      })
      .collect::<Vec<(usize, usize)>>();
  ...
}
```

# Optimised for reversed pattern
To minimize effort, search for reversed patterns starting at 'S' and reverse the target pattern for these cases:

```rust
fn part1() -> i32 {
    ...
    let pairs = [('X', ['M', 'A', 'S']), ('S', ['A', 'M', 'X'])];
    pairs
        .into_iter()
        .map(|(start_char, pattern)| {
            let starts = map
                .iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|&(_, &c)| c == start_char)
                        .map(move |(j, _)| (i, j))
                })
                .collect::<Vec<(usize, usize)>>();

            starts
                .iter()
                .map(|start| find_pattern(start, &map, &pattern))
                .sum::<i32>()
        })
        .sum()
    ...
}
```

# Search pattern
The pattern can be found in four directions: down, right, up-right, and down-right. By zipping direction vectors `dx` and `dy`, you can iterate through all possible directions:

```rust
fn find_pattern(start: &(usize, usize), map: &[Vec<char>], pattern: &[char]) -> i32 {
    // 4 possible directions:
    // down, right, rightup, rightdown
    let dx = [0, 1, 1, 1];
    let dy = [1, 0, -1, 1];
    ...

    dx.iter()
    .zip(dy.iter())
    .filter(...)
    .count() as i32
}
```

During pattern searching, we also have to make sure the positions are within the grid using a `bound_check` function as follows:

```rust
fn bound_check(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < height && y >= 0 && y < width
}
```

The final `find_pattern()` function is written as follows, We iterate through all possible directions, and for each direction, we verify whether advancing three steps matches the specified pattern. If the pattern is found, the direction is retained; otherwise, it is discarded. Finally, we calculate the total number of directions where the pattern was successfully located:

```rust
fn find_pattern(start: &(usize, usize), map: &[Vec<char>], pattern: &[char]) -> i32 {
    // 4 possible directions:
    // down, right, rightup, rightdown
    let dx = [0, 1, 1, 1];
    let dy = [1, 0, -1, 1];

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    dx.iter()
        .zip(dy.iter())
        .filter(|(&i, &j)| {
            let mut new_x = start.0 as i32;
            let mut new_y = start.1 as i32;
            pattern.iter().all(|&pat| {
                new_x += i;
                new_y += j;
                bound_check(new_x, new_y, width, height)
                    && map[new_x as usize][new_y as usize] == pat
            })
        })
        .count() as i32
}
```

# Vectors, arrays and slices
An interesting point worth noting is the choice of parameter types for the find_pattern function. Following advice from `cargo clippy`, it is recommended to use slices (`&[String]`) instead of vectors (`Vec<String>`). Let’s explore the reasons behind this recommendation:

* Vectors like `Vec<String>` are used when you have an unknown number of items you prefer to allocate on the heap at runtime. However, using vectors also means it requires an extra pointer to access the data, compared to arrays and slices. A dynamic collection type also indicate more frequent reallocations with no capacity is specified beforehand, so if you know the capacity requirement of your vector, it is recommended to set the capacity of the vector first (e.g., `Vec::with_capacity()`) to avoid unnecessary reallocation.
* Arrays like `[String; N]` are allocated on the stack at run-time, it is suitable to use when you know the total number of items and faster direct access to the items. It works well for a small, fixed-sized collection.
* Slices are a view into portions of a vector or an array, the type is written as `&[String]`, it is well-suited when we only need read-access, and do not know the exact size at compile time. In function parameters, we prefer to use Slices like `&[String]` because it makes the function parameter more generic, for example, the Slice type `&[String]` can accept any type that can be borrowed as a Slice of Strings, these include Vectors `Vec<String>`, Arrays `[String; N]` and Slices `&[String]`.

# Full program
```rust
use std::io::{BufRead, BufReader};

fn bound_check(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < height && y >= 0 && y < width
}

fn find_pattern(start: &(usize, usize), map: &[Vec<char>]) -> i32 {
    // 8 possible directions:
    // up, down, left, right, leftup, rightup, leftdown, rightdown
    let dx = [0, 0, -1, 1, -1, 1, -1, 1];
    let dy = [-1, 1, 0, 0, -1, -1, 1, 1];
    let pattern = ['M', 'A', 'S'];

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    dx.iter()
        .zip(dy.iter())
        .filter(|(&i, &j)| {
            let mut new_x = start.0 as i32;
            let mut new_y = start.1 as i32;
            pattern.iter().all(|&pat| {
                new_x += i;
                new_y += j;
                bound_check(new_x, new_y, width, height)
                    && map[new_x as usize][new_y as usize] == pat
            })
        })
        .count() as i32
}

fn part1() -> i32 {
  let f = std::fs::File::open(<FILE_PATH>).unwrap();
  let r = BufReader::new(f);

  let map = r
      .lines()
      .map(|line| line.unwrap().chars().collect::<Vec<_>>())
      .collect::<Vec<_>>();

  let starts = map
      .iter()
      .enumerate()
      .flat_map(|(i, row)| {
          row.iter()
              .enumerate()
              .filter(|&(_, &c)| c == 'X')
              .map(move |(j, _)| (i, j))
      })
      .collect::<Vec<(usize, usize)>>();
  starts.iter().map(|start| find_pattern(start, &map)).sum()
}
```

# Part 2
# Problem statement
Find occurrences of `"MAS"` arranged in an X shape, allowing for reversed strings. The structure looks like this:

```
M.S
.A.
M.S
```

There are four possible orientations for this pattern.

# Finding all starting positions
Start from `'A'` in the middle and eliminate positions on the edges of the grid to reduce computation:

```rust
fn part2() -> i32 {
    let starts = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &c)| c == 'A')
                .map(move |(j, _)| (i, j))
                .filter(|&(i, j)| i != 0 && i != map.len() - 1 && j != 0 && j != map[0].len() - 1)
        })
        .collect::<Vec<(usize, usize)>>();
    ...
}
```

# Search pattern from A
The `find_pattern2()` function is a slight modification of the previous `find_pattern()` function. There are four possible orientations of the `MAS` pattern: up-left, up-right, down-left, and down-right.

For each starting position `A`, at most two `MAS` patterns can be identified to form an `X-MAS`. If fewer than two `MAS` patterns are found around a starting position, no `X-MAS` pattern can be formed. Therefore, the total count of `MAS` patterns must be divided by 2 at the end to account for this pairing:

```rust
fn find_pattern2(start: &(usize, usize), map: &[Vec<char>]) -> i32 {
    // possible postions of (M, S)
    let dx = [(-1, 1), (1, -1), (1, -1), (-1, 1)];
    let dy = [(-1, 1), (-1, 1), (1, -1), (1, -1)];

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    (dx.iter()
        .zip(dy.iter())
        .filter(|&(i, j)| {
            let (i_m, j_m) = (i.0 + start.0 as i32, j.0 + start.1 as i32);
            let (i_s, j_s) = (i.1 + start.0 as i32, j.1 + start.1 as i32);
            bound_check(i_m, j_m, width, height)
                && bound_check(i_s, j_s, width, height)
                && map[i_m as usize][j_m as usize] == 'M'
                && map[i_s as usize][j_s as usize] == 'S'
        })
        .count()
        / 2) as i32
}
```

# Full program
```rust
use std::io::{BufRead, BufReader};

fn bound_check(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < height && y >= 0 && y < width
}

fn find_pattern2(start: &(usize, usize), map: &[Vec<char>]) -> i32 {
    // possible postions of (M, S)
    let dx = [(-1, 1), (1, -1), (1, -1), (-1, 1)];
    let dy = [(-1, 1), (-1, 1), (1, -1), (1, -1)];

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    (dx.iter()
        .zip(dy.iter())
        .filter(|&(i, j)| {
            let (i_m, j_m) = (i.0 + start.0 as i32, j.0 + start.1 as i32);
            let (i_s, j_s) = (i.1 + start.0 as i32, j.1 + start.1 as i32);
            bound_check(i_m, j_m, width, height)
                && bound_check(i_s, j_s, width, height)
                && map[i_m as usize][j_m as usize] == 'M'
                && map[i_s as usize][j_s as usize] == 'S'
        })
        .count()
        / 2) as i32
}

fn part2() -> i32 {
    let f = std::fs::File::open(<FILE_PATH>).unwrap();
    let r = BufReader::new(f);

    let map = r
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let starts = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &c)| c == 'A')
                .map(move |(j, _)| (i, j))
        })
        .collect::<Vec<(usize, usize)>>();

    starts.iter().map(|start| find_pattern2(start, &map)).sum()
}

```