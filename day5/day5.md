# Part 1: Problem Description
The input consists of two sections:
1. **Page Ordering Rules**: Rules are given as `X|Y`, indicating that page `X` must be printed before page `Y`.
2. **Updates**: Each update contains a list of page numbers. The task is to verify if the pages in each update adhere to the specified ordering rules.

For instance, given an update `75, 29, 13`, consider the case where `13` must be printed before `75`. If there are no ordering constraints between `75` and `29` or between `29` and `13`, simply checking adjacent entries might fail to ensure correctness. 

# Handling Input Splitting
To efficiently split the input into two sections, we use `split_once`, which separates the input into a string for rules and another for updates.

# Limitations of Naïve Sorting
```rust
// Naive approach
fn compute(buf: &str) -> i32 {
    let (s1, s2) = buf.split_once("\n\n").unwrap();
    // orders: key needs to print before values
    let mut orders: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in s1.lines() {
        let (x, y) = line.split_once("|").unwrap();
        orders
            .entry(x.parse::<i32>().unwrap())
            .or_insert(HashSet::new())
            .insert(y.parse::<i32>().unwrap());
    }

    let mut p1 = 0;
    for line in s2.lines() {
        let mut vec = line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if vec
            .windows(2)
            .all(|w| !orders.contains_key(&w[1]) || !orders[&w[1]].contains(&w[0]))
        {
            p1 += vec[vec.len() / 2];
        }
    }

    p1
}
```

The naïve approach checks adjacency only:
```rust
if vec
    .windows(2)
    .all(|w| !orders.contains_key(&w[1]) || !orders[&w[1]].contains(&w[0]))
```

However, this fails in cases where:
1. Some pairs are not directly constrained.
2. Transitivity needs to be enforced.

Example:
Rules:
```
13|75
75|29
```
Update:
```
75,29,13
```
The naive method incorrectly assumes validity. Instead, transitivity shows `13` must precede `75`, and the update fails.

# Topological Sorting
A **topological sort** resolves this by respecting all constraints:
* Each node depends on zero or more nodes.
* Nodes with zero dependencies are processed first.
* Dependencies of processed nodes are reduced until the entire graph is sorted.
* Topological sort requires an **acyclic graph**.

Here are the key steps:
1. Compute **in-degrees** for all nodes (number of dependencies).
2. . Use a queue to process nodes with zero in-degree.
3. Decrement in-degrees of dependents as nodes are processed.
4. Continue until all nodes are sorted.

```rust
fn topological_sort(
    v_dep_k: &HashMap<i32, HashSet<i32>>,
    k_dep_v: &HashMap<i32, HashSet<i32>>,
    update: &[i32],
) -> Vec<i32> {
    // Compute in-degrees for nodes in the update
    let mut in_degrees = update
        .iter()
        .map(|&k| match k_dep_v.get(&k) {
            Some(v) => (
                k,
                v.into_iter().filter(|vs| update.contains(vs)).count() as i32,
            ),
            None => (k, 0),
        })
        .collect::<HashMap<i32, i32>>();

    // Initialize the queue with nodes having zero in-degree
    let mut queue = in_degrees
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(&k, _)| k)
        .collect::<Vec<i32>>();

    // Perform topological sorting
    let mut sorted = vec![];
    while let Some(cur_node) = queue.pop() {
        sorted.push(cur_node);
        if let Some(neighbours) = v_dep_k.get(&cur_node) {
            for neighbour in neighbours.iter().filter(|n| update.contains(n)) {
                in_degrees.entry(*neighbour).and_modify(|v| *v -= 1);
                if in_degrees[&neighbour] == 0 {
                    queue.push(*neighbour);
                }
            }
        }
    }

    sorted
}
```

# Full program
```rust
fn topological_sort(
    v_dep_k: &HashMap<i32, HashSet<i32>>,
    k_dep_v: &HashMap<i32, HashSet<i32>>,
    update: &[i32],
) -> Vec<i32> {
    // Compute in-degrees for nodes in the update
    let mut in_degrees = update
        .iter()
        .map(|&k| match k_dep_v.get(&k) {
            Some(v) => (
                k,
                v.into_iter().filter(|vs| update.contains(vs)).count() as i32,
            ),
            None => (k, 0),
        })
        .collect::<HashMap<i32, i32>>();

    // Initialize the queue with nodes having zero in-degree
    let mut queue = in_degrees
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(&k, _)| k)
        .collect::<Vec<i32>>();

    // Perform topological sorting
    let mut sorted = vec![];
    while let Some(cur_node) = queue.pop() {
        sorted.push(cur_node);
        if let Some(neighbours) = v_dep_k.get(&cur_node) {
            for neighbour in neighbours.iter().filter(|n| update.contains(n)) {
                in_degrees.entry(*neighbour).and_modify(|v| *v -= 1);
                if in_degrees[&neighbour] == 0 {
                    queue.push(*neighbour);
                }
            }
        }
    }

    sorted
}

fn compute(buf: &str) -> i32 {
    let (s1, s2) = buf.split_once("\n\n").unwrap();
    let mut k_deps_v: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut v_deps_k: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in s1.lines() {
        // x needs to be printed before y, y depends on x
        // Parse rules into dependency graphs
        let (x, y) = line.split_once("|").unwrap();
        v_deps_k
            .entry(x.parse::<i32>().unwrap())
            .or_insert(HashSet::new())
            .insert(y.parse::<i32>().unwrap());
        k_deps_v
            .entry(y.parse::<i32>().unwrap())
            .or_insert(HashSet::new())
            .insert(x.parse::<i32>().unwrap());
    }

    let mut p1 = 0;
    for line in s2.lines() {
        let vec = line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let sorted = topological_sort(&v_deps_k, &k_deps_v, &vec);
        if sorted.eq(&vec) {
            p1 += vec[vec.len() / 2];
        }
    }

    p1
}
```

# Part 2
In part 2, instead of discarding unsorted updates, sort them and compute the sum of their middle numbers.

High-level solution:
* Compare the sorted result with the original update.
* If they differ, add the middle element of the sorted update to `p2`.

# Example Output
Input:
```
13|75
75|29

75,29,13
```
Result:
- **Part 1**: `p1 = 0` (invalid update)
- **Part 2**: `p2 = 29` (sorted update's middle element)

# Full program
```rust
fn compute(buf: &str) -> (i32, i32) {
    let (s1, s2) = buf.split_once("\n\n").unwrap();

    let mut k_deps_v: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut v_deps_k: HashMap<i32, HashSet<i32>> = HashMap::new();

    // Parse rules into dependency graphs
    for line in s1.lines() {
        let (x, y) = line.split_once("|").unwrap();
        v_deps_k
            .entry(x.parse::<i32>().unwrap())
            .or_insert(HashSet::new())
            .insert(y.parse::<i32>().unwrap());
        k_deps_v
            .entry(y.parse::<i32>().unwrap())
            .or_insert(HashSet::new())
            .insert(x.parse::<i32>().unwrap());
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for line in s2.lines() {
        let vec = line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let sorted = topological_sort(&v_deps_k, &k_deps_v, &vec);

        if sorted == vec {
            p1 += vec[vec.len() / 2];
        } else {
            p2 += sorted[sorted.len() / 2];
        }
    }

    (p1, p2)
}
```
