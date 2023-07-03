use std::io::stdin;

pub fn get_input() -> String {
    let mut input = String::new();
    println!("Input:");
    stdin().read_line(&mut input).expect("Failed to read input");
    input
}
