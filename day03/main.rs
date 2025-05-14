use std::fs;
use std::collections::HashSet;
use std::ops::Add;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point (i32, i32);

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

fn part1(input: &str) -> usize {
    let mut seen = HashSet::<Point>::new();
    let mut cur_pos: Point = Point(0, 0);
    seen.insert(cur_pos); //implicity visiting the first house as per instruction.

    let chars = input.chars();
    for part in chars {
        //println!("{part}");
        let delta = match part {
            '^' => Point(0, 1),
            '>' => Point(1, 0),
            '<' => Point(-1, 0),
            'v' => Point(0, -1),
            _ => continue,
        };

        cur_pos = cur_pos + delta;
        seen.insert(cur_pos);
    }

    return seen.len() as usize;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected a file!");
    let part1_ans = part1(&input);
    println!("Houses visited: {}", part1_ans);

}

