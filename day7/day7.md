# Part 1
# Problem statement
Validate whether a given set of expressions produces the expected results. Starting with an equation's expected value and a list of numbers, use two operators, `+` and `*`, to compute the expected value. The operators have equal precedence, meaning the evaluation is strictly left-to-right.

In the following example, it is a valid equation because `292 = 11 + 6 * 16 + 20`.
```
292: 11 6 16 20
```

# Rust zero-cost abstraction
To embrace idiomatic Rust practices, I plan to use a custom struct for better abstraction. As suggested by an experienced Rustacean, Rust’s zero-cost abstraction allows for high-level constructs with no runtime overhead compared to low-level implementations.

Hence is a customised `Equation` struct with two member fields: an expected `value` and a list of `numbers`. 

```rust
struct Equation {
    value: i64,
    numbers: Vec<i64>,
}
```

# Recursion
The Equation struct includes a method `is_valid` to check if the expected value can be computed using the `+` and `*` operators.

This solution uses recursion. An equation like `a ? b ? c` is valid if either `a + (b ? c)` or `a * (b ? c)` is valid. This approach breaks the problem `a ? b ? c` into smaller sub-problems like `b ? c`, eventually reaching the base case where no numbers remain.

```rust
impl Equation {
    fn is_valid(&self, v: i64, nums: &[i64]) -> bool {
        if nums.len() == 0 {
            return self.value == v;
        }
        let (h, t) = (nums[0], &nums[1..]);
        return self.is_valid(v * h, t) || self.is_valid(v + h, t);
    }
}
```

# Full program
```rust
struct Equation {
    value: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn is_valid(&self, v: i64, nums: &[i64]) -> bool {
        if nums.len() == 0 {
            return self.value == v;
        }
        let (h, t) = (nums[0], &nums[1..]);
        return self.is_valid(v * h, t) || self.is_valid(v + h, t);
    }
}
fn parse_inputs() -> Vec<Equation> {
    let f = std::fs::File::open(FILE_PATH).unwrap();
    let r = BufReader::new(f);
    r.lines()
        .map(|line| {
            let l = line.unwrap();
            let mut parts_iter = l.split(':');
            Equation {
                value: parts_iter.next().unwrap().trim().parse::<i64>().unwrap(),
                numbers: parts_iter
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            }
        })
        .collect::<Vec<Equation>>()
}

fn part1() -> i64 {
    parse_inputs()
        .iter()
        .map(|eqt| {
            if eqt.is_valid_revert(eqt.numbers[0], &eqt.numbers[1..]) {
                eqt.value
            } else {
                0
            }
        })
        .sum::<i64>()
}
```

# Part 2
# Problem statement
Same as in Part 1, but with an additional operator `concat` (`||`), which concatenates two numbers into one. For example, `x || y` results in `xy`.

# Simple recursion extension
* Using the `+` Operator:
  *  Works with owned values (e.g., `String`), but **does not work** with string slices (`&str`).
  *  Concatenates the right-hand-side (RHS) string into the left-hand-side (LHS) string, reusing the LHS buffer. However, reallocation may occur if the LHS buffer is insufficient, making it less efficient for multi-string concatenation.

* Using the `format!` Macro:
  * Highly expressive and readable syntax, works with both owned (`String`) and borrowed (`&str`) values.
  * Less efficient compared to other methods: the macro is complex and less likely to be inlined by the compiler, and the compiler preallocates a `String` with an estimated capacity, which may lead to either memory waste (overestimation) or additional reallocations (underestimation).

* Using the `push_str()` Method:
  * Efficient for incremental concatenation as `push_str()` works directly on a `String`.
  * Rust strings internally use `Vec<u8>`. When the buffer is insufficient, it automatically doubles the capacity, balancing memory use and minimizing future reallocations.
  * Amortized time complexity is **O(1)**.
  * The syntax can be less intuitive compared to other methods.

* Using `concat()` on Arrays:
  * Works with both owned and borrowed values.
  * Efficient, as `concat()` calculates the total memory needed at runtime and preallocates it, avoiding future reallocations.
  * Requires creating an array of strings, which can make the syntax less intuitive.
  * Strings of different types may require explicit type conversions to fit into the same array.

* Using `join()` on Iterables:
  * Similar to `concat()`, but allows the inclusion of a delimiter between strings, offering greater flexibility.
  * Efficient memory allocation for concatenation, though slightly slower than `concat()` due to the inclusion of separators.
  * Allocates extra memory for the separators.

For this question, we use `concat()` to join the two strings into one:
```rust
impl Equation {
    ...
    fn is_valid2(&self, v: i64, nums: &[i64]) -> bool {
        if nums.len() == 0 {
            return self.value == v;
        }
        let (h, t) = (nums[0], &nums[1..]);
        return self.is_valid2(v * h, t)
            || self.is_valid2(v + h, t)
            || self.is_valid2(
                [v.to_string(), h.to_string()]
                    .concat()
                    .parse::<i64>()
                    .unwrap(),
                &t,
            );
    }
}
```

# Optimisation: reversed evaluation to prune branches earlier
In the simple recursion case, we have to search all branches, which gives 3^n worst case number of cases for n operators. To effectively prune the unnecessary branches earlier. 
* Instead of `*`, we can reverse the computation by applying `/` instead, since if we cannot get an integer after a division, the branch is no longer a possible candidate. 
* Instead of concat(`||`), i.e., `a || b = ab`, we can reverse the computation, start from the expected result and check if `ab` ends with `b` and continue the computation with `a`.

```rust
impl Equation {
    ...
    fn is_valid2_revert(&self, v: i64, revert_nums: &[i64]) -> bool {
        if revert_nums.len() == 1 {
            return v == revert_nums[0];
        }
        let (h, t) = (revert_nums[0], &revert_nums[1..]);
        let sub_opt = if v - h >= 0 {
            self.is_valid2_revert(v - h, t)
        } else {
            false
        };
        let div_opt = if v % h == 0 {
            self.is_valid2_revert(v / h, t)
        } else {
            false
        };

        let num_digits = h.ilog10() + 1;
        let ends_with = v % (10_i64.pow(num_digits)) == h;
        let concat_opt = if ends_with {
            self.is_valid2_revert((v - h) / (10_i64.pow(num_digits)), t)
        } else {
            false
        };
        return sub_opt || div_opt || concat_opt;
    }
}
```

# Full program
```rust
struct Equation {
    value: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn is_valid2_revert(&self, v: i64, revert_nums: &[i64]) -> bool {
        if revert_nums.len() == 1 {
            return v == revert_nums[0];
        }
        let (h, t) = (revert_nums[0], &revert_nums[1..]);
        let sub_opt = if v - h >= 0 {
            self.is_valid2_revert(v - h, t)
        } else {
            false
        };
        let div_opt = if v % h == 0 {
            self.is_valid2_revert(v / h, t)
        } else {
            false
        };

        let num_digits = h.ilog10() + 1;
        let ends_with = v % (10_i64.pow(num_digits)) == h;
        let concat_opt = if ends_with {
            self.is_valid2_revert((v - h) / (10_i64.pow(num_digits)), t)
        } else {
            false
        };
        return sub_opt || div_opt || concat_opt;
    }
}
fn parse_inputs() -> Vec<Equation> {
    let f = std::fs::File::open(FILE_PATH).unwrap();
    let r = BufReader::new(f);
    r.lines()
        .map(|line| {
            let l = line.unwrap();
            let mut parts_iter = l.split(':');
            Equation {
                value: parts_iter.next().unwrap().trim().parse::<i64>().unwrap(),
                numbers: parts_iter
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            }
        })
        .collect::<Vec<Equation>>()
}
fn part2() -> i64 {
    parse_inputs()
        .iter()
        .map(|eqt| {
            let mut nums_copy = eqt.numbers.clone();
            nums_copy.reverse();
            if eqt.is_valid2_revert(eqt.value, &nums_copy) {
                eqt.value
            } else {
                0
            }
        })
        .sum::<i64>()
}
```