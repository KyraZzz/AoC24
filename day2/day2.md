# AoC 24 in Rust: Day 2
The time has come! The annual Advent of Code programming challenge is just around the corner. This year, I plan to tackle the challenge using the Rust programming language. I see it as a fantastic opportunity to deepen my understanding of idiomatic Rust practices.

I'll document my journey to share with the community, hoping it serves as a helpful resource for programmers who want to learn Rust in a fun and engaging way.

# Part 1
# Problem statement
Given multiple vectors of non-negative integers, a vector is considered valid if:
* The numbers are in strictly ascending or descending order, and
* The difference between any two adjacent numbers is between 1 and 3 (inclusive).

For example:
* The first vector is not valid because it is neither in strict ascending nor descending order, and the difference between the first and second numbers exceeds 3.
* The second vector is valid because it is in strict descending order, and all adjacent differences fall within the range of 1 to 3.
```
3 8 6 8 10 12 15 -> not valid
58 55 54 53 51 50 -> valid
```

# Parsing inputs
In [Day 1](https://www.reddit.com/r/adventofcode/comments/1h5teio/2024_day_1_rust_tutorial_the_rusty_way_to/), we discussed various ways to read inputs from a text file. Check it out if you're curious about why I chose a buffered reader for this task.

For this question, we parse each report as a vector of non-negative integers and then check its validity. Finally, we aggregate the results by counting the number of valid reports.

```rust
use std::io::{BufRead, BufReader};
fn part1() -> u32 {
    let f = std::fs::File::open(<FILE_PATH>).unwrap();
    let r = BufReader::new(f);

    let res: usize = r
        .lines()
        .filter(|line: &Result<String, Error>| {
            let report = line
                .as_ref() // WHY? -> &Result<&String, &Error>
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            report_is_valid(&report)
        })
        .count();

    res as u32
}
```
You might wonder why we need to invoke `as_ref()` to convert the type of line from `&Result<String, Error>` to `Result<&String, &Error>`.

First, let’s explore an interesting aspect of ownership in `map` and `filter` operations:

* The `map(|v: T| transform(v))` operation transforms a value into a new one, potentially of a different type. Since the original value is no longer needed after transformation, map takes ownership of it.
* The `filter(|v: &T| inspect(v))` operation, on the other hand, merely inspects the value to determine if it meets certain conditions. If it does, the original value is preserved; otherwise, it is discarded. Since filter does not modify the value, it only borrows an immutable reference instead of taking ownership.

Now, returning to the original question: `line` is of type `&Result<String, Error>`. We cannot directly call `unwrap()` on this type because `unwrap()` on a `Result` requires ownership of the `Result`. To resolve this, we use `as_ref()` to convert the value from `&Result<String, Error>` to `Result<&String, &Error>`. This allows us to work with references instead of taking ownership, which is more efficient in this context.

# Check validity of the report using a sliding window
A report is considered valid if it satisfies two conditions: it must be in strict ascending or descending order, and the difference between adjacent values must be within the inclusive range of 1 to 3.

The `windows(size)` method creates a `Windows` struct, which acts as an iterator over overlapping subslices. The implementation of `Windows` is highly efficient due to the following reasons:
* Lazy evaluation: It only computes the window when `next()` is invoked.
* Memory efficiency: It returns immutable references and only keeps track of the current index and the window's size.

```rust
fn report_is_valid(report: &Vec<u32>) -> bool {
    if report.len() == 1 {
        return true;
    }
    // check ascending or descending order
    let is_ascending = report.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = report.windows(2).all(|w| w[0] >= w[1]);
    if !is_ascending && !is_descending {
        return false;
    }

    // check diff is within range [1,3]
    let is_valid_range = report
        .windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)
        .all(|x| x.abs() >= 1 && x.abs() <= 3);

    is_valid_range
}
```

# Final program
```rust
use std::io::{BufRead, BufReader};

fn report_is_valid(report: &Vec<u32>) -> bool {
    if report.len() == 1 {
        return true;
    }
    // check ascending or descending order
    let is_ascending = report.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = report.windows(2).all(|w| w[0] >= w[1]);
    if !is_ascending && !is_descending {
        return false;
    }

    // check diff is within range [1,3]
    let is_valid_range = report
        .windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)
        .all(|x| x.abs() >= 1 && x.abs() <= 3);

    is_valid_range
}

fn part1() -> u32 {
    let f = std::fs::File::open(<FILE_PATH>).unwrap();
    let r = BufReader::new(f);

    let res: usize = r
        .lines()
        .filter(|line| {
            let report = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            report_is_valid(&report)
        })
        .count();

    res as u32
}
```

# Part 2
# Problem statement
This is similar to part 1, but with the added flexibility of tolerating one error in each vector. Specifically, if the vector satisfies all the rules from part 1 after removing a single entry, we consider it valid. For example, the vector becomes valid if we remove the first 8.

```
3 ~8~ 6 8 10 12 15 -> valid
```

# Concatenating vector slices
The brute-force approach is to clone the vector and remove each item one at a time, checking if the resulting vector satisfies both conditions by invoking `report_is_valid()`. However, using `remove()` on a cloned vector is inefficient because it requires reallocation after each removal.

```rust
fn report_is_tolerable(report: &Vec<u32>) -> bool {
    if report_is_valid(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut report_copy = report.clone();
        report_copy.remove(i);
        if report_is_valid(&report_copy) {
            return true;
        }
    }
    false
}
```
Instead, we can use `concat()` to concatenate two vector slices into a new vector. This approach allocates memory for the new vector in a single operation, minimizing reallocations and processing each item only once. As a result, it has a time complexity of `O(m + n)`, where `m` and `n` are the number of items in each slice, respectively.

Note that `concat()` creates a new vector from the slices without taking ownership of the original items. Therefore, we need to provide slice references when calling `concat()`.

```rust
fn report_is_tolerable(report: &Vec<u32>) -> bool {
    ...
    for i in 0..report.len() {
        let report_copy = [&report[0..i], &report[i + 1..]].concat();
        if report_is_valid(&report_copy) {
            return true;
        }
    }
    ...
}
```

# Final program
```rust
use std::io::{BufRead, BufReader};

fn report_is_valid(report: &Vec<u32>) -> bool {
    if report.len() == 1 {
        return true;
    }
    // check ascending or descending order
    let is_ascending = report.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = report.windows(2).all(|w| w[0] >= w[1]);
    if !is_ascending && !is_descending {
        return false;
    }

    // check diff is within range [1,3]
    let is_valid_range = report
        .windows(2)
        .map(|w| w[1] as i32 - w[0] as i32)
        .all(|x| x.abs() >= 1 && x.abs() <= 3);

    is_valid_range
}

fn report_is_tolerable(report: &Vec<u32>) -> bool {
    if report_is_valid(report) {
        return true;
    }
    for i in 0..report.len() {
        let report_copy = [&report[0..i], &report[i + 1..]].concat();
        if report_is_valid(&report_copy) {
            return true;
        }
    }
    false
}

fn part2() -> u32 {
    let f = std::fs::File::open(<FILE_PATH>).unwrap();
    let r = BufReader::new(f);

    let res: usize = r
        .lines()
        .filter(|line| {
            let report = line
                .as_ref()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            report_is_tolerable(&report)
        })
        .count();

    res as u32
}
```