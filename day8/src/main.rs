use grid::Grid;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

fn bound_check(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < (height as i32) && y >= 0 && y < (width as i32)
}

fn part1() {
    let f = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day8/test/inputs.txt").unwrap();
    let r = BufReader::new(f);
    let m = r
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let height = m.len();
    let width = m[0].len();
    let grid = Grid::from_vec(m.into_iter().flatten().collect(), width);

    let mut char_to_pos: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut res = HashSet::new();
    grid.indexed_iter()
        .filter(|((_, _), &c)| c != '.')
        .for_each(|((i, j), c)| {
            let binding = vec![];
            let mut set = char_to_pos.get(c).unwrap_or(&binding).clone();
            set.push((i as i32, j as i32));
            // res.insert((i as i32, j as i32));
            char_to_pos.insert(*c, set);
        });

    for (char, pos_pairs) in char_to_pos.iter() {
        for i in 0..pos_pairs.len() {
            for j in i + 1..pos_pairs.len() {
                let p1 = pos_pairs[i];
                let p2 = pos_pairs[j];
                let diff = (p2.0 - p1.0, p2.1 - p1.1);
                let new1 = (p1.0 - diff.0, p1.1 - diff.1);
                let new2 = (p2.0 + diff.0, p2.1 + diff.1);
                for new in [new1, new2] {
                    if bound_check(new.0, new.1, width, height) && !res.contains(&new) {
                        res.insert(new);
                    }
                }
            }
        }
    }
    println!("{:?}", res.len());
}

fn main() {
    let f = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day8/test/inputs.txt").unwrap();
    let r = BufReader::new(f);
    let m = r
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let height = m.len();
    let width = m[0].len();
    let grid = Grid::from_vec(m.into_iter().flatten().collect(), width);

    let mut char_to_pos: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut res = HashSet::new();
    grid.indexed_iter()
        .filter(|((_, _), &c)| c != '.')
        .for_each(|((i, j), c)| {
            let binding = vec![];
            let mut set = char_to_pos.get(c).unwrap_or(&binding).clone();
            set.push((i as i32, j as i32));
            char_to_pos.insert(*c, set);
        });

    for (char, pos_pairs) in char_to_pos.iter() {
        for i in 0..pos_pairs.len() {
            for j in i + 1..pos_pairs.len() {
                let p1 = pos_pairs[i];
                let p2 = pos_pairs[j];
                let diff = (p2.0 - p1.0, p2.1 - p1.1);
                if !res.contains(&p1) {
                    res.insert(p1);
                }
                if !res.contains(&p2) {
                    res.insert(p2);
                }

                let mut new1 = (p1.0 - diff.0, p1.1 - diff.1);
                while bound_check(new1.0, new1.1, width, height) {
                    if !res.contains(&new1) {
                        res.insert(new1);
                    }

                    new1 = (new1.0 - diff.0, new1.1 - diff.1);
                }

                let mut new2 = (p2.0 + diff.0, p2.1 + diff.1);
                while bound_check(new2.0, new2.1, width, height) {
                    if !res.contains(&new2) {
                        res.insert(new2);
                    }

                    new2 = (new2.0 + diff.0, new2.1 + diff.1);
                }
            }
        }
    }
    println!("{:?}", res.len());
}
