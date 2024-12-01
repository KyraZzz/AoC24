use std::collections::HashMap;
use std::io::{BufRead, BufReader};
fn part1() -> i32 {
    let f = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day1/test/part1.txt").unwrap();
    let r = BufReader::new(f);
    let arrays = r
        .lines()
        .map(|line| {
            line.unwrap()
                .split("   ")
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut firsts = arrays.iter().map(|x| x.get(0).unwrap()).collect::<Vec<_>>();
    firsts.sort();
    let mut seconds = arrays.iter().map(|x| x.get(1).unwrap()).collect::<Vec<_>>();
    seconds.sort();

    let mut res = 0;
    for (first, second) in firsts.into_iter().zip(seconds.into_iter()) {
        res += (second - first).abs();
    }
    res
}

fn part2() -> i32 {
    let f = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day1/test/part1.txt").unwrap();
    let r = BufReader::new(f);
    let arrays = r
        .lines()
        .map(|line| {
            line.unwrap()
                .split("   ")
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let firsts = arrays
        .iter()
        .map(|x| x.get(0).unwrap().to_owned())
        .collect::<Vec<i32>>();
    let seconds = arrays
        .iter()
        .map(|x| x.get(1).unwrap().to_owned())
        .collect::<Vec<i32>>();

    let mut occurrences = HashMap::new();

    for num in seconds.into_iter() {
        occurrences.insert(num, occurrences.get(&num).unwrap_or(&0) + 1);
    }
    let mut score = 0;
    for num in firsts.into_iter() {
        score += num * occurrences.get(&num).unwrap_or(&0);
    }
    score
}

fn main() {
    let res1 = part1();
    println!("part 1: {}", res1);

    let res2 = part2();
    println!("part 2: {}", res2);
}
