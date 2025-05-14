use std::{fs, result};
use std::collections::HashMap;
#[derive(Debug, Clone)]
struct Gate {
    w1: String,
    w2: String,
    op: String,
}

impl From<&str> for Gate {
    fn from(s: &str) -> Gate {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts[0] == "NOT" {
            Gate {
                op: parts[0].to_string(),
                w1: parts[1].to_string(),
                w2: "".to_string(),
            } 
        } else if parts.len() == 1 {
            Gate {
                op: "NONE".to_string(),
                w1: parts[0].to_string(),
                w2: "".to_string(),
          }  
        } else {
            Gate {
                op: parts[1].to_string(),
                w1: parts[0].to_string(),
                w2: parts[2].to_string(),
            }
        }
    }
}

struct Circuit {
    commands: HashMap<String, Gate>,
    values: HashMap<String, u16>
}

impl Circuit {
    fn new() -> Self {
        Self {
            commands: HashMap::new(),
            values: HashMap::new()
        }
    }

    fn process(&mut self, node: String) -> u16 {
        if node.chars().all(|c: char| c.is_ascii_digit()) {
            return node.parse::<u16>().unwrap();
        }

        if self.values.contains_key(&node) {
            return *self.values.get(&node).unwrap();
        }

        let g =  self.commands.get(&node).unwrap().clone();
        let v1 = self.process(g.w1);
        let op = g.op.as_str();
        let res = match op { 
            "NONE" => v1,
            "NOT" => !v1,
            _ => {
                let v2 = self.process(g.w2);
                match op {
                    "OR" => v1 | v2,
                    "AND" => v1 & v2,
                    "RSHIFT" => v1 >> v2,
                    "LSHIFT" => v1 << v2,
                    _ => panic!("Unknown operator: {}", op),
                }
            }
        };

        self.values.insert(node.to_string(), res);
        res
    }




}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected a file!");
    
    let mut circuit = Circuit::new();

    for line in input.lines() {
        let (input, output) = line.split_once(" -> ").unwrap();
        circuit.commands.insert(output.to_string(), input.into());        
    }
    
    println!("Part 1: {:?}", circuit.process("a".to_string()));
    
    //lazy way.... lol
    let mut input = fs::read_to_string("input.txt").expect("Expected a file!");
    input += "\n956 -> b"; 
    let mut circuit = Circuit::new();
    
    for line in input.lines() {
        let (input, output) = line.split_once(" -> ").unwrap();
        circuit.commands.insert(output.to_string(), input.into());        
    }
    println!("Part 2: {:?}", circuit.process("a".to_string()));


}
