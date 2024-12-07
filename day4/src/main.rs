use std::io::{BufRead, BufReader};

static FILE_PATH: &'static str = "/Users/Kyra_ZHOU/24AoC/AoC24/day4/test/inputs.txt";
fn bound_check(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < height && y >= 0 && y < width
}

fn find_pattern(start: &(usize, usize), map: &[Vec<char>], pattern: &[char]) -> i32 {
    // 4 possible directions:
    // down, right, rightup, rightdown
    let dx = [0, 1, 1, 1];
    let dy = [1, 0, -1, 1];

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    dx.iter()
        .zip(dy.iter())
        .filter(|(&i, &j)| {
            let mut new_x = start.0 as i32;
            let mut new_y = start.1 as i32;
            pattern.iter().all(|&pat| {
                new_x += i;
                new_y += j;
                bound_check(new_x, new_y, width, height)
                    && map[new_x as usize][new_y as usize] == pat
            })
        })
        .count() as i32
}

fn part1() -> i32 {
    let f = std::fs::File::open(FILE_PATH).unwrap();
    let r = BufReader::new(f);

    let map = r
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let pairs = [('X', ['M', 'A', 'S']), ('S', ['A', 'M', 'X'])];
    pairs
        .into_iter()
        .map(|(start_char, pattern)| {
            let starts = map
                .iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|&(_, &c)| c == start_char)
                        .map(move |(j, _)| (i, j))
                })
                .collect::<Vec<(usize, usize)>>();

            starts
                .iter()
                .map(|start| find_pattern(start, &map, &pattern))
                .sum::<i32>()
        })
        .sum()
}

fn find_pattern2(start: &(usize, usize), map: &[Vec<char>]) -> i32 {
    // possible postions of (M, S)
    let dx = [(-1, 1), (1, -1), (1, -1), (-1, 1)];
    let dy = [(-1, 1), (-1, 1), (1, -1), (1, -1)];

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    (dx.iter()
        .zip(dy.iter())
        .filter(|&(i, j)| {
            let (i_m, j_m) = (i.0 + start.0 as i32, j.0 + start.1 as i32);
            let (i_s, j_s) = (i.1 + start.0 as i32, j.1 + start.1 as i32);
            bound_check(i_m, j_m, width, height)
                && bound_check(i_s, j_s, width, height)
                && map[i_m as usize][j_m as usize] == 'M'
                && map[i_s as usize][j_s as usize] == 'S'
        })
        .count()
        / 2) as i32
}

fn part2() -> i32 {
    let f = std::fs::File::open(FILE_PATH).unwrap();
    let r = BufReader::new(f);

    let map = r
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let starts = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &c)| c == 'A')
                .map(move |(j, _)| (i, j))
                .filter(|&(i, j)| i != 0 && i != map.len() - 1 && j != 0 && j != map[0].len() - 1)
        })
        .collect::<Vec<(usize, usize)>>();

    starts.iter().map(|start| find_pattern2(start, &map)).sum()
}

fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
}
