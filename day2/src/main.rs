use std::io::{BufRead, BufReader};
fn part1() {
    let f = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day2/test/input.txt").unwrap();
    let r = BufReader::new(f);
    let l = r
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for line in l {
        let length = line.len();
        if length == 1 {
            res += 1;
            continue;
        }

        let sign = if (line[1] - line[0]) > 0 { 1 } else { -1 };
        let filtered = line
            .windows(2)
            .map(|w| w[1] - w[0])
            .filter(|x| x * sign > 0)
            .filter(|x| x.abs() >= 1 && x.abs() <= 3)
            .collect::<Vec<_>>();
        if filtered.len() == length - 1 {
            res += 1;
            continue;
        }
    }
    println!("{}", res);
}
fn is_valid(line: &Vec<i32>) -> bool {
    let length = line.len();
    if length == 1 {
        return true;
    }

    let sign = if (line[1] - line[0]) > 0 { 1 } else { -1 };
    let filtered = line
        .windows(2)
        .map(|w| w[1] - w[0])
        .filter(|x| x * sign > 0)
        .filter(|x| x.abs() >= 1 && x.abs() <= 3)
        .collect::<Vec<_>>();
    if filtered.len() == length - 1 {
        return true;
    }
    false
}
fn main() {
    let f = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day2/test/input.txt").unwrap();
    let r = BufReader::new(f);
    let l = r
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for line in l {
        if is_valid(&line) {
            res += 1;
        } else {
            for i in 0..line.len() {
                let mut new_copy = line.clone();
                new_copy.remove(i);
                if is_valid(&new_copy) {
                    res += 1;
                    break;
                }
            }
        }
    }
    println!("{}", res);
}
