use regex::Regex;

fn part1() -> i32 {
    let input = include_str!("../test/input.txt");
    let re = Regex::new(r"mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\)").unwrap();

    let mut res = 0;
    for m in re.captures_iter(input) {
        let op1 = m.name("op1").unwrap().as_str().parse::<i32>().unwrap();
        let op2 = m.name("op2").unwrap().as_str().parse::<i32>().unwrap();
        res += op1 * op2;
    }

    res
}
fn part2() -> i32 {
    let input = include_str!("../test/input.txt");
    let re = Regex::new(r"(mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\))|(don't\(\))|(do\(\))").unwrap();

    let mut res = 0;

    let mut valid = true;
    for m in re.captures_iter(input) {
        let op = m.get(0).unwrap().as_str();
        valid = if op.starts_with("do()") {
            true
        } else if op.starts_with("don't()") {
            false
        } else {
            valid
        };
        if valid && op.starts_with("mul") {
            let op1 = m.name("op1").unwrap().as_str().parse::<i32>().unwrap();
            let op2 = m.name("op2").unwrap().as_str().parse::<i32>().unwrap();

            res += op1 * op2;
        }
    }

    res
}

fn main() {
    println!("part 1: {:?}", part1());
    println!("part 2: {:?}", part2());
}
