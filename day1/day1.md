# AoC 24 in Rust: Day 1
The time has come! The annual Advent of Code programming challenge is just around the corner. This year, I plan to tackle the challenge using the Rust programming language. I see it as a fantastic opportunity to deepen my understanding of idiomatic Rust practices.

I'll document my journey to share with the community, hoping it serves as a helpful resource for programmers who want to learn Rust in a fun and engaging way.

## Part1
### Problem statement
*The problem statement is often a bit verbose, so I'll simplify it as much as possible in this blog. However, I encourage you to read the original problem statement if you're in the mood for some festive Christmas vibes.*

Given two vectors of non-negative integers, pair the numbers in non-descending order. For each pair, calculate the distance (absolute difference) between the numbers and sum up all the distances.

In the following example, we pair up 1 (smallest in left) with 3 (smallest in right) which gives a distance of 2, then pair up 2 (second smallest in left) with 3 (second smallest in right) which gives a distance of 1, and so on. The total distance is `2 + 1 + 0 + 1 + 2 + 5 = 11`.
```
3   4
4   3
2   5
1   3
3   9
3   3
```

### Read strings from a txt file 
The first challenge we encountered was determining how to correctly read inputs into the program. Depending on the specific use case, we select an appropriate method for reading strings from a file.

The `include_str!(<FILE_PATH>)` macro is a powerful tool in Rust that reads UTF-8 encoded file contents at compile time and embeds them as a string literal in the compiled binary. This approach offers several advantages: 
- The string literal is a static str, meaning it has a static lifetime and is available throughout the entire program.
- If the specified file is not found, the program will panic at compile time, preventing runtime errors.
- The file path is resolved relative to the current file at compile time. Consequently, the file only needs to exist during compilation, as its contents are embedded in the binary. This allows the program to execute independently of the original file's location.
Despite its benefits, `include_str!` is not suitable for handling large files. Embedding large files as string literals in the compiled binary can increase compilation time and complexity, inflate the binary file size and raise memory usage of the program when it loads into memory. Typical use cases for include_str! include configuration files or other small, constant content that needs to be accessible throughout the program.

To read the entire contents of a file as a string at runtime, we can use the `std::fs::read_to_string(<FILE_PATH>)` function. This convenient function handles opening the file, reading its contents, and converting them into a string, while also propagating potential errors. The function supports relative file path resolution, but it resolves paths based on the current working directory of the executing process. This behavior aligns with the conventions of many other programming languages but may cause confusion if you are accustomed to using `include_str!(<RELATIVE_FILE_PATH>)`, which resolves paths relative to the source file at compile time. The return type of `std::fs::read_to_string` is `Result<String, std::io::Error>`. If the file is not found or cannot be read, it returns a runtime I/O error. This function is well-suited for reading small to medium-sized files that only need to be accessible for a certain lifetime and do not require embedding into the compiled binary.

To handle large files efficiently and read them line by line, a buffered reader is typically used. A buffered reader maintains an internal buffer, usually 8 KB in size. It reads a large chunk of data (8 KB) in a single system call, significantly reducing the number of expensive system calls required to read the entire file. Buffered readers also provide a convenient `lines()` method, which allows iteration over all the lines in the file. This method reuses a single string for each line, further optimizing memory usage during the reading process.

In today's challenge, we use a buffered reader to read in the file and process it line by line.

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
fn part1() {
    let f: File = File::open(<FILE_PATH>).unwrap();
    let r: BufReader<File> = BufReader::new(f);
    for line in r.lines() {
        println!("{:?}", line);
    }
    ...
}
```

### Parse inputs into two non-negative integer vectors
To parse the inputs, we first read them as non-negative integers. Then, we construct two vectors: one containing the first number from each line and the other containing the second number from each line.
```
3   4
4   3
2   5
1   3
3   9
3   3
```
Each line contains two numbers separated by three white spaces. Consider a more general case with unknown number of white spaces in-between the two numbers, there are two common methods to handle this:
- `split_whitespace()`: This method splits the string into parts separated by any amount of whitespace.
- `split(" ").filter(|s| !s.is_empty())`: This method splits the string at single spaces. Any consecutive spaces result in empty strings (""), which must be filtered out.

In Rust, the Iterator pattern and Map-Reduce pattern are generally recommended because they offer several advantages: (1) They enable parallel processing and minimize overhead; (2) They modularize operations, providing a consistent and uniform interface regardless of the underlying collection type; (3) They align with Rust’s design philosophy, resulting in code that is easier to read, maintain, and reason about.

To parse the input into a nested vector of non-negative integers, one approach is to first create a nested vector of integers, then separate it into two vectors. However, this requires three full scans of the data. Can we achieve the same result in a single scan?
```rust
fn part1() {
    ...
    let numbers = r
        .lines()
        .map(|line| {
            line.expect("Failed to read line")
                .split_whitespace()
                .map(|s| s.parse::<u32>().expect("Failed to parse number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut firsts = numbers
        .iter()
        .map(|x| x.get(0).unwrap())
        .collect::<Vec<_>>();
    let mut seconds = numbers
        .iter()
        .map(|x| x.get(1).unwrap())
        .collect::<Vec<_>>();
    ...
}
```

Yes! After processing each line and parsing the values into unsigned integers, we return a tuple instead of a complete vector. This approach allows us to construct two vectors directly in a single scan.
```rust
fn part1() {
    ...
    let (mut firsts, mut seconds) = r
        .lines()
        .map(|line| {
            {
                let vec = line
                    .expect("Failed to read line")
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().expect("Failed to parse number"))
                    .collect::<Vec<u32>>();
                (
                    vec.get(0).expect("Failed to fetch first number").to_owned(),
                    vec.get(1)
                        .expect("Failed to fetch second number")
                        .to_owned(),
                )
            }
        })
        .collect::<(Vec<u32>, Vec<u32>)>();
    ...
}
```

### Sort a vector in non-descending order
-`<vector>.sort()`: Implements a stable merge sort, which preserves the order of equivalent items. It has a worst-case time complexity of `O(n log n)` and a space complexity of `O(n)`.
-`<vector>.sort_unstable()`: Optimized for speed by sacrificing stability. It requires fewer item-swaps and uses a pattern-defeating quicksort with efficient pivot selection. This method has a worst-case time complexity of `O(n log n)` and a space complexity of `O(log n)`.

```rust
fn part1() {
    ...
    firsts.sort_unstable();
    seconds.sort_unstable();
    ...
}
```

### Compute element-wise distances between two vectors
Next, we compute the element-wise distances between each pair in the two vectors. To convert a collection into an iterator for processing, there are two main options:
- `<collection>.into_iter()`: Transfers ownership of the items to the new iterator, consuming the original collection. After use, the original collection is no longer accessible. This method is ideal when the collection is no longer needed and allows more flexible modification of the items. Note that `into_iter()` is implicitly called in a for loop when iterating over the items of a collection.
- `<collection>.iter()`: Returns borrowed immutable references to the items in the collection. This option is suitable when you only need to inspect the items without consuming or modifying them, leaving the original collection intact and available for further use.

Here we can choose to consume the original collection because we no longer need them after computing the sum.
```rust
fn part1() -> i32 {
    ...
    let res: i32 = firsts
        .into_iter()
        .zip(seconds.into_iter())
        .map(|(first, second)| (second as i32 - first as i32).abs())
        .sum();
    res
}
```

### Final program
*I replaced all instances of expect(msg) with unwrap() for the sake of conciseness. We'll dive into error handling next time!* 
```rust
fn part1() -> i32 {
    let f: File = std::fs::File::open(<FILE_PATH>).unwrap();
    let r: BufReader<File> = BufReader::new(f);

    let (mut firsts, mut seconds) = r
        .lines()
        .map(|line| {
            let vec = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (
                vec.get(0).unwrap().to_owned(),
                vec.get(1).unwrap().to_owned(),
            )
        })
        .collect::<(Vec<u32>, Vec<u32>)>();

    firsts.sort_unstable();
    seconds.sort_unstable();

    let res: i32 = firsts
        .into_iter()
        .zip(seconds.into_iter())
        .map(|(first, second)| (second as i32 - first as i32).abs())
        .sum();
    res
}
```

## Part 2
### Problem statement
Each daily challenge consists of two parts. The second part of the problem becomes accessible only after successfully solving the first part.

The second part typically presents a variation of the first. In this case, given two vectors of non-negative integers, we multiply each entry in the left vector by the number of times that entry appears in the right vector. In the following example, we have 3 * 3 + 4 * 1 + 2 * 0 + 1 * 0 + 3 * 3 + 3 * 3 = 31.
```
3   4
4   3
2   5
1   3
3   9
3   3
``` 

### Use HashMap as a counter
We count the occurrences of each number in the second vector using a HashMap.

An interesting detail to note is the ownership of items in the HashMap when we invoke `get(&K) -> Option<&V>`. This method returns an Option type. If the key exists, it returns an immutable reference to the value; otherwise, it returns `None`.

If we are confident that the key exists in the HashMap, we can safely use `unwrap()`. Otherwise, we should provide a default value with `unwrap_or(default)` or handle the error explicitly. The default value provided in `unwrap_or(default)` must be of the same type as the unwrapped value (e.g., `&v` in this case).

It is generally recommended to avoid working with references directly when the value implements the `Copy` trait (e.g., primitive types like integers and floats). Dealing with multi-level references can be error-prone, and raw values are more flexible for modification. Most simple types that implement the `Copy` trait are efficiently copied, so performance concerns are minimal. Therefore, instead of using `.get(&num).unwrap_or(&0)`, we can use `.get(&num).copied().unwrap_or(0)`.

```rust
fn part2 () -> u32 {
    ... // unsorted vector firsts, seconds
    let mut occurrences = HashMap::new();
    for num in seconds.into_iter() {
        occurrences.insert(num, occurrences.get(&num).copied().unwrap_or(0) + 1);
    }
    let score = firsts
        .into_iter()
        .map(|num| num * occurrences.get(&num).copied().unwrap_or(0))
        .sum();
    score
}
```

### Final program
```rust
fn part2() -> u32 {
    let f = std::fs::File::open(<FILE_PATH>).unwrap();
    let r = BufReader::new(f);

    let (firsts, seconds) = r
        .lines()
        .map(|line| {
            let vec = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (
                vec.get(0).unwrap().to_owned(),
                vec.get(1).unwrap().to_owned(),
            )
        })
        .collect::<(Vec<u32>, Vec<u32>)>();

    let mut occurrences = HashMap::new();
    for num in seconds.into_iter() {
        occurrences.insert(num, occurrences.get(&num).copied().unwrap_or(0) + 1);
    }
    let score = firsts
        .into_iter()
        .map(|num| num * occurrences.get(&num).copied().unwrap_or(0))
        .sum();
    score
}
```