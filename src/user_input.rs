use std::io;

pub fn user_input() {
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word ('stop' to exit)");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered: {}", input);
    }
    println!("Equisdé");
}