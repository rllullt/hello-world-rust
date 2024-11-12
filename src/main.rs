fn main() {
    let mut message = String::from("Name: Óscar, Height: ");
    message.clear();
    let mut height = 190;
    height = 189;
    println!("{}{}", message, height);

    el_maybe();
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
