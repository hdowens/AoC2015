use std::fs;

fn mem_length(line: &str) -> (usize, usize) {
    let mut mem_tot = 0;
    let v = line[1..line.len()-1].chars().collect::<Vec<char>>();
    let mut index = 0;
    while index < v.len() {
        mem_tot += 1;
        if v[index] == '\\' {
            if v[index+1] == 'x' {
                index += 4;
            } else {
                index += 2;
            }
         } else {
            index += 1;
         }
    }
    (line.len(), mem_tot)
}

fn encode_string(line: &str) -> (usize, usize) {
    let mut enc = String::new();
    
    for c in line.chars() {
        match c {
            '"' => {
                enc.push('\\');
                enc.push(c);
            }
            '\\' => {
                enc.push('\\');
                enc.push(c);    
            }
            _ => enc.push(c),
        }
    }
    (enc.len() + 2, line.len())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected a file!");

    let mut p1_tot = 0;
    for line in input.lines() {
        let (tot, mem) = mem_length(line);
        p1_tot += tot - mem;
    }
    
    let mut p2_tot = 0;
    for line in input.lines() {
        let (tot, mem) = encode_string(line);
        p2_tot += tot - mem;
    }
    

    println!("Part 1 : {p1_tot}");
    println!("Part 2 : {p2_tot}");

}
