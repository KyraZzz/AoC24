use core::num;
use std::io::{BufRead, BufReader};
static FILE_PATH: &'static str = "/Users/Kyra_ZHOU/24AoC/AoC24/day7/test/inputs.txt";
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

fn part1() -> i64 {
    parse_inputs()
        .iter()
        .map(|eqt| {
            if eqt.is_valid(eqt.numbers[0], &eqt.numbers[1..]) {
                eqt.value
            } else {
                0
            }
        })
        .sum::<i64>()
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

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
