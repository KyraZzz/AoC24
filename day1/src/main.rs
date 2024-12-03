use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn part1() -> i32 {
    let f: File = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day1/test/part1.txt").unwrap();
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

fn part2() -> u32 {
    let f = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day1/test/part1.txt").unwrap();
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

fn main() {
    let res1 = part1();
    println!("part 1: {}", res1);

    let res2 = part2();
    println!("part 2: {}", res2);
}
