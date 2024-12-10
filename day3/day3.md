# AoC 24 in Rust: Day 3
# Part 1
# Problem statement
Given a string of instructions, identify all valid `mul(x,y)` operations where `x` and `y` are non-negative integers. Then, calculate the sum of the products for all such `mul` operations.

In the following example, we have valid mul instructions `mul(2,4)`, `mul(5,5)`, `mul(11,8)` and `mul(8,5)`, the total sum of products is 2 * 4 + 5 * 5 + 11 * 8 + 8 * 5 = 161
```
xmul(2,4)%&mul[3,7]don't()_mul(5,5)+mul(32,64]do()(mul(11,8)mul(8,5))
```

# The Regex crate and useful methods
Yes, the time has come—we need to tackle pattern matching. The easiest way (without writing a custom parser) is to leverage regular expressions. Rust provides an efficient crate called Regex that supports searching and replacing within UTF-8 encoded strings. It offers a worst-case time complexity of `O(mn)` where `m` is the pattern length, and `n` is the length of the input string, or "haystack" (using the correct terminology here).

In Rust, a crate is the smallest unit of code organization, similar to a package or library in other programming languages. There are two types of crates:

* Binary executable crates: Contain a `main()` function and can be executed directly.
* Library crates: Provide reusable functionalities that can be shared across projects but are not directly executable.

The Regex crate is a library crate.

In Part 1, we want to capture the pattern `mul(X,Y)` where `X` and `Y` are non-negative integers. A simple regular expression like `r"mul\([0-9]+,[0-9]+\)"` can be used to match this pattern. Once we find matches, we extract `X` and `Y`, multiply them, and compute the sum of all resulting products.

To achieve this, use `Regex::new(<PATTERN>)` to compile the regular expression `<PATTERN>` into an optimized internal representation, `r`. During compilation, Regex parses and validates the pattern, ensuring there are no format errors and allocating memory for efficient reuse. Because the compilation process is computationally expensive, avoid recompiling the same pattern repeatedly (e.g., in a loop).

The Regex crate offers several methods for pattern searching in a haystack. Below are some key options:

# `r.find_iter(haystack: &str) -> Matches`
* Returns an iterator of successive non-overlapping matches (`Match`) for the pattern `r` within the haystack.
* A Match contains: (1) the start and end byte offsets of the match in the haystack; (2) the actual matching substring.
* Note: The returned byte offsets are crucial, as Regex works only with UTF-8 encoded strings. In UTF-8, characters can range from 1 to 4 bytes, so byte offsets always align with UTF-8 code-point boundaries.
* Finding `X` and `Y` within a Match object can be a bit tedious. For example, in the pattern `mul(X,Y)`, the sub-pattern `X,Y` starts at byte offset 4 (inclusive) and ends at byte offset `m.len() - 1` (exclusive).So let us take a look at another option `r.captures_iter()`.

```rust
fn part1() -> u32 {
    ...
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();

    re.find_iter(input)
        .map(|m| {
            m.as_str()[4..m.len() - 1]
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .product::<u32>()
        })
        .sum()
}
```

# `r.captures_iter(haystack: &str) -> Captures`
* Returns an iterator of successive non-overlapping captures (`Capture`) from the pattern r within the haystack.
* This method allows users to extract different parts or groups in the pattern easily, with the option to name groups for clarity. Each group is of type `Match` and can be accessed either by its index or its name (if specified in the pattern). 
* A group is defined using parentheses and can optionally be named. To name a group, use `(?<name>...)`, where `name` is the group name, followed by the regular expression for that group. For example, `(?<op1>[0-9]+)` defines a group named `op1`, which matches one or more digits `([0-9]+)`.
* Performance considerations: the `r.find()` method is faster and less resource-intensive. Use `r.captures_iter()` only when you need to access specific capture group matches within the pattern.
* Here are three possible implementations using `captures_iter()`:

```rust
// Method 1:
fn part1() -> u32 {
    ...
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut res = 0;
    for c in re.captures_iter(input) {
        // group index 0 represents the overall match, so the two groups we are interested in are indexed at 1 and 2
        res += &c[1].parse::<u32>().unwrap() * &c[2].parse::<u32>().unwrap();
    }
    res
}
```
```rust
// Method 2:
fn part1() -> u32 {
    ...
    let re = Regex::new(r"mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\)").unwrap();
    let mut res = 0;
    for c in re.captures_iter(input) {
        // fetch groups using their specified names in the pattern
        res += &c["op1"].parse::<u32>().unwrap() * &c["op2"].parse::<u32>().unwrap();
    }
    res
} 
```
```rust
// Method 3:
fn part1() -> u32 {
    ...
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut res = 0;
    // extract() returns (full_match, [group_match;N])
    for (_, [op1, op2]) in re.captures_iter(input).map(|c| c.extract()) {
        res += op1.parse::<u32>().unwrap() * op2.parse::<u32>().unwrap()
    }
    res
} 
```

# Final program
```rust
fn part1() -> u32 {
    let input = include_str!(<FILE_PATH>);
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut res = 0;
    for (_, [op1, op2]) in re.captures_iter(input).map(|c| c.extract()) {
        res += op1.parse::<u32>().unwrap() * op2.parse::<u32>().unwrap()
    }
    res
} 
```

## Part 2
### Problem statement
Two additional instructions, `do()` and `don't()`, are introduced:
* The calculator starts in a valid state.
* `do()` ensures the machine is in a valid state if it is not already.
* `don't()` transitions the machine to an invalid state if it is not already.

When encountering a valid mul instruction:
* If the machine is in a valid state, proceed as in Part 1 by adding the product to the result.
* If the machine is in an invalid state, skip the instruction.

The following example disabled `mul(5,5)` due to the preceding `don't()` instruction. The last two valid mul instructions are enabled due to the preceding `do()` instruction. Hence, the result becomes 2 * 4 + 11 * 8 + 8 * 5 = 136.
```
xmul(2,4)%&mul[3,7]don't()_mul(5,5)+mul(32,64]do()(mul(11,8)mul(8,5))
```

# Or-ing multiple patterns in one Regex expression
Similar to Part 1, but this time we add two additional patterns, `don't\(\)` and `do\(\)`, using a logical OR.

To differentiate between the patterns during iteration, we can name the groups. This allows us to identify which pattern each Capture instance belongs to—whether it corresponds to a `don't`, `do`, or `mul(X, Y)` instruction.

# Final program
```rust
fn part2() -> i32 {
    let input = include_str!(<FILE_PATH>);
    let re = Regex::new(r"(mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\))|(?<dt>don't\(\))|(?<d>do\(\))")
        .unwrap();

    let mut res = 0;
    let mut valid = true;
    for m in re.captures_iter(input) {
        if let Some(_) = m.name("d") {
            valid = true;
        } else if let Some(_) = m.name("dt") {
            valid = false;
        }
        if valid {
            if let (Some(op1), Some(op2)) = (m.name("op1"), m.name("op2")) {
                res += op1.as_str().parse::<i32>().unwrap() * op2.as_str().parse::<i32>().unwrap();
            }
        }
    }
    res
}
```