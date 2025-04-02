use std::fs;
use std::cmp::min;
use regex::Regex;

fn box_surface_area(l: u32, w: u32, h: u32) -> u32 {
    let min_side = min(l * w, min(w * h, h * l));
    2 * l * w + 2 * w * h + 2 * h * l + min_side
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected a file!");
    let re_pat = Regex::new(r"\d+").unwrap();

    let part1_total: u32 = input
        .lines()
        .filter_map(|line| {
            let nums: Vec<u32> = re_pat.find_iter(line)
                .filter_map(|m| m.as_str().parse::<u32>().ok())
                .collect();
            match nums.as_slice() {
                [l, w, h] => Some(box_surface_area(*l, *w, *h)),
                _ => None,
            }
        })
        .sum();

    let part2_total: u32 = input
        .lines()
        .filter_map(|line| {
            let mut nums: Vec<u32> = re_pat.find_iter(line)
                .filter_map(|m| m.as_str().parse::<u32>().ok())
                .collect();
            if nums.len() != 3 {
                return None;
            }
            nums.sort();
            let ribbon = 2 * nums[0] + 2 * nums[1]; // Smallest perimeter
            let volume = nums[0] * nums[1] * nums[2];
            Some(ribbon + volume)
        })
        .sum();

    println!("Part 1: {}", part1_total);
    println!("Part 2: {}", part2_total);
}
