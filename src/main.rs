mod user_input;

fn main() {
    // Most Rust line codes end with a semicolon («the expression is over»)
    let mut message = String::from("Name: Óscar, Height: ");
    message.clear();
    let mut height = 190;
    height = 189;
    // println notes:
    // println! llama a una macro Rust (por el `!`, sino, sería función)
    // Es decir, usar ! significa llamar una macro
    println!("{}{}", message, height);

    el_maybe();

    println!("User input:");
    user_input::user_input();
}

fn el_maybe() {
    let maybe_number = Some(42);
    if let Some(number) = maybe_number {
        println!("The number is: {:?}", number);
    }
    else {
        println!("There is no number");
    }
}
