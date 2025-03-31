use std::fs;
use std::cmp::min;
use regex::Regex;

fn box_surface_area(l: u32, w: u32, h: u32) -> u32 {
    let min_side = min(l*w,min(w*h,h*l));
    return (2*l*w + 2*w*h + 2*h*l) + min_side;
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected a file!");
    
    let re_pat = Regex::new(r"\d+").unwrap();    

    let mut total = 0; 
    for line in input.lines() {
        let nums: Vec<u32> = re_pat.find_iter(line)
            .filter_map(|m| m.as_str().parse::<u32>().ok())
            .collect();
        let surface_area = box_surface_area(
            nums[0], nums[1], nums[2]
        );
        total += surface_area;
        
    }
    println!("Total: {total}");
}
