use regex::Regex;

fn part1() -> u32 {
    let input = include_str!("../test/input.txt");
    // let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    // let re = Regex::new(r"mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\)").unwrap();
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    // re.find_iter(input)
    //     .map(|m| {
    //         m.as_str()[4..m.len() - 1]
    //             .split(",")
    //             .map(|n| n.parse::<u32>().unwrap())
    //             .product::<u32>()
    //     })
    //     .sum()

    let mut res = 0;
    // for c in re.captures_iter(input) {
    //     res += &c["op1"].parse::<u32>().unwrap() * &c["op2"].parse::<u32>().unwrap();
    // }
    for (_, [op1, op2]) in re.captures_iter(input).map(|c| c.extract()) {
        res += op1.parse::<u32>().unwrap() * op2.parse::<u32>().unwrap()
    }
    res
}
fn part2() -> i32 {
    let input = include_str!("../test/input.txt");
    let re = Regex::new(r"(mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\))|(?<dt>don't\(\))|(?<d>do\(\))")
        .unwrap();

    let mut res = 0;
    let mut valid = true;
    for m in re.captures_iter(input) {
        if let Some(_) = m.name("d") {
            valid = true;
        } else if let Some(_) = m.name("dt") {
            valid = false;
        }
        if valid {
            if let (Some(op1), Some(op2)) = (m.name("op1"), m.name("op2")) {
                res += op1.as_str().parse::<i32>().unwrap() * op2.as_str().parse::<i32>().unwrap();
            }
        }
    }
    res
}

fn main() {
    println!("part 1: {:?}", part1());
    println!("part 2: {:?}", part2());
}
