use md5;

fn main() {

    let starter = "bgvyzdsv";
    for i in 0..5_000_000 {
        
        let test = starter.to_string() + &i.to_string();
        //println!("{:?}", test);
        let digest = md5::compute(test);
        //println!("{:?}", digest);
        let dig_str: String = digest.iter().map(|byte| format!("{:02x}", byte)).collect();
        if &dig_str[0..6] == "000000" {
            println!("Winnar: {:?} {}", dig_str, i);
        }
    }

}
