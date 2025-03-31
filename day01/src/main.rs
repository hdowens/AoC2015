use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Issue reading file");
    
    let mut floor = 0;
    let mut index = 0;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!()
        }

        index += 1;
        if floor == -1 {
            println!("Part 2: {index}");
        }
    }

    println!("Part 1: {floor}");

}