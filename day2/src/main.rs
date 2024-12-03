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
    let f = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day2/test/input.txt").unwrap();
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
    let f = std::fs::File::open("/Users/Kyra_ZHOU/24AoC/AoC24/day2/test/input.txt").unwrap();
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

fn main() {
    println!("part1: {:?}", part1());
    println!("part2: {:?}", part2());
}
