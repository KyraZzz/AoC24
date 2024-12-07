use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

static FILE_PATH: &'static str = "/Users/Kyra_ZHOU/24AoC/AoC24/day6/test/inputs.txt";

fn bound_check(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < height && y >= 0 && y < width
}

fn init_map() -> Vec<Vec<char>> {
    let f = std::fs::File::open(FILE_PATH).unwrap();
    let r = BufReader::new(f);
    r.lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_start(m: &[Vec<char>]) -> (i32, i32) {
    let start = m
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '^')
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .collect::<Vec<(i32, i32)>>()[0];
    // let mut start = (0, 0);
    // for (i, row) in m.iter().enumerate() {
    //     for (j, &c) in row.iter().enumerate() {
    //         if c == '^' {
    //             start = (i as i32, j as i32);
    //         }
    //     }
    // }
    start
}

fn out_of_grid(start: &(i32, i32, usize), m: &[Vec<char>]) -> usize {
    let width = m[0].len();
    let height = m.len();
    // directions in an anticlockwise way: up, right, down, left
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut cur_x, mut cur_y, mut cur_idx) = start;
    let mut hits = HashSet::new();
    loop {
        hits.insert((cur_x, cur_y));

        let cur_step = dirs[cur_idx];
        let (new_x, new_y) = (cur_x + cur_step.0, cur_y + cur_step.1);
        if !bound_check(new_x, new_y, width as i32, height as i32) {
            return hits.len();
        }
        if m[new_x as usize][new_y as usize] == '#' {
            cur_idx = (cur_idx + 1) % 4;
        } else {
            (cur_x, cur_y) = (new_x, new_y);
        }
    }
}

fn out_of_grid2(start: &(i32, i32, usize), m: &[Vec<char>]) -> Option<i32> {
    let width = m[0].len();
    let height = m.len();
    // directions in an anticlockwise way
    // up, right, down, left
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut cur_x, mut cur_y, mut cur_idx) = start;
    // optimisation: using a 2D array rather than a hashset
    let mut hits = vec![vec![[false; 4]; m[0].len()]; m.len()];
    // let mut hits = HashSet::new();
    loop {
        // if hits.contains(&(cur_x, cur_y, cur_idx)) {
        //     return None;
        // }
        // hits.insert((cur_x, cur_y, cur_idx));
        if hits[cur_x as usize][cur_y as usize][cur_idx] {
            return None;
        }
        hits[cur_x as usize][cur_y as usize][cur_idx] = true;

        let cur_step = dirs[cur_idx];
        let (new_x, new_y) = (cur_x + cur_step.0, cur_y + cur_step.1);
        if !bound_check(new_x, new_y, width as i32, height as i32) {
            // return Some(
            //     hits.iter()
            //         .map(|(i, j, _)| (*i, *j))
            //         .collect::<HashSet<(i32, i32)>>(),
            // );
            return Some(
                (hits
                    .iter()
                    .enumerate()
                    .flat_map(|(i, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|&(_, col)| col.iter().any(|&b| b))
                            .map(move |(j, _)| (i, j))
                    })
                    .collect::<Vec<(usize, usize)>>()
                    .len()) as i32,
            );
        }
        if m[new_x as usize][new_y as usize] == '#' {
            cur_idx = (cur_idx + 1) % 4;
        } else {
            (cur_x, cur_y) = (new_x, new_y);
        }
    }
}

fn part1() -> u32 {
    let mut m = init_map();
    let start = find_start(&m);
    m[start.0 as usize][start.1 as usize] = '.';

    out_of_grid(&(start.0, start.1, 0), &m) as u32
}

fn part2() -> u32 {
    let mut m = init_map();
    let start = find_start(&m);
    m[start.0 as usize][start.1 as usize] = '.';

    let width = m[0].len();
    let height = m.len();

    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut cur_idx = 0;
    let (mut cur_x, mut cur_y) = (start.0, start.1);

    // let mut visited = HashSet::new();
    // visited.insert((cur_x, cur_y));
    let mut visited = vec![vec![false; width]; height];

    let mut res = 0;
    loop {
        visited[cur_x as usize][cur_y as usize] = true;
        let cur_step = dirs[cur_idx];
        let (new_x, new_y) = (cur_x + cur_step.0, cur_y + cur_step.1);
        if !bound_check(new_x, new_y, width as i32, height as i32) {
            break;
        }
        if m[new_x as usize][new_y as usize] == '#' {
            cur_idx = (cur_idx + 1) % 4;
        } else {
            if (new_x, new_y) != start && !visited[new_x as usize][new_y as usize] {
                m[new_x as usize][new_y as usize] = '#';
                if out_of_grid2(&(cur_x, cur_y, cur_idx), &m).is_none() {
                    res += 1;
                }
                m[new_x as usize][new_y as usize] = '.';
            }
            (cur_x, cur_y) = (new_x, new_y);
        }
    }
    res as u32
}

fn main() {
    println!("part1: {:?}", part1());
    println!("part2: {:?}", part2());
}
