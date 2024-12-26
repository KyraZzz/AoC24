#![feature(is_sorted)]
use std::{
    collections::{HashMap, HashSet},
    io::{BufReader, Read},
};

fn topological_sort(
    v_dep_k: &HashMap<i32, HashSet<i32>>,
    k_dep_v: &HashMap<i32, HashSet<i32>>,
    update: &[i32],
) -> Vec<i32> {
    // compute in-degree of each node, i.e., how many nodes it depends on
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

    // a node can be put into the sorted vector if it has a 0 in-degree
    let mut queue = in_degrees
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(&k, _)| k)
        .collect::<Vec<i32>>();

    let mut sorted = vec![];
    while !queue.is_empty() {
        let cur_node = queue.pop().unwrap();
        sorted.push(cur_node);
        if let Some(neighbours) = v_dep_k.get(&cur_node) {
            for neighbour in neighbours.iter().filter(|n| update.contains(n)) {
                in_degrees.entry(*neighbour).and_modify(|v| *v -= 1);
                if in_degrees[&neighbour] == 0 {
                    queue.push(neighbour.clone());
                }
            }
        }
    }

    sorted
}

fn compute(buf: &str) -> (i32, i32) {
    let (s1, s2) = buf.split_once("\n\n").unwrap();
    let mut k_deps_v: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut v_deps_k: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in s1.lines() {
        // x needs to be printed before y, y depends on x
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
        if sorted.eq(&vec) {
            p1 += vec[vec.len() / 2];
        } else {
            p2 += sorted[sorted.len() / 2];
        }
    }

    (p1, p2)
}

fn simple_compute(buf: &str) -> (i32, i32) {
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
    let mut p2 = 0;
    for line in s2.lines() {
        let mut vec = line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if vec.is_sorted_by(|a, b| !orders.contains_key(&b) || !orders[b].contains(a)) {
            p1 += vec[vec.len() / 2];
        } else {
            vec.sort_by(|a, b| (!orders.contains_key(b) || orders[b].contains(a)).cmp(&true));
            p2 += vec[vec.len() / 2];
        }
    }

    (p1, p2)
}

fn main() {
    let f = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day5/test/example.txt").unwrap();
    let mut r = BufReader::new(f);
    let mut buf = String::new();
    let _ = r.read_to_string(&mut buf);
    let res_simple = simple_compute(&buf);
    println!("Simple compute:");
    println!("part1: {:?}", res_simple.0);
    println!("part2: {:?}", res_simple.1);

    let res = compute(&buf);
    println!("Topological sort:");
    println!("part1: {:?}", res.0);
    println!("part2: {:?}", res.1);
}
