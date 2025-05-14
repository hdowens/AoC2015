use std::fs;
use rustc_lexer::unescape::unescape_str;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected a file!");

    let mut len_tot: usize = 0; 
    let mut len_mem: usize = 0;
    for line in input.lines() {
        let len = line.len();
        println!("{line} : {len}");
        len_tot += len;

        let trimmed = &line[1..line.len() - 1];
        let mut in_memory = String::new();
        unescape_str(trimmed, &mut |_, result| {
            match result {
                Ok(c) => in_memory.push(c),
                Err(e) => panic!("Unescape error: {:?}", e),
            }
        });
    
        len_mem += in_memory.len();

    }
    
    println!("Total length: {len_tot}");
    println!("Total memory length: {len_mem}");
    println!("Answer: {}", len_tot - len_mem);

}
