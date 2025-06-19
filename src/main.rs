mod user_input;
mod fibo;
mod factorial;
mod collatz;

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

    let n = 7; // inmutable, para mutabilidad usar 'let mut'
    println!("Fibonacci recursivo de {n}:");
    println!("fibo({n}) = {}", fibo::fibo(n));  // fibo debe ser pública!!

    println!("match expression (pattern matching) + quick and dirty debug with dbg! macro");
    let mut val = 1;
    dbg!(val);
    match val {
        1 => {
            println!("one");
            dbg!(val);
        },  // Note: blocks are expressions
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => {
            println!("something else");
        }
    }

    let flag = true;
    val = match flag {
        true => 1,
        false => 0,
    };
    println!("The value of {flag} is {val}");

    println!("For loop");
    for x in 1..=5 {  // 1..5 = [1, 2, 3, 4], 1..=5 = [1, 2, 3, 4, 5]
        dbg!(x);
    }

    println!("gcd function definition and execution:");
    fn gcd(a: u32, b: u32) -> u32 {
        if b > 0 { gcd(b, a % b) } else { a }
    }
    dbg!(gcd(142, 52));

    val = 6;
    println!("Factorial de {val}");
    println!("{val}! = {}", factorial::factorial(val));

    let n1 = 11;  // collatz length of 15
    println!("Collatz length of (n1={n1}): {}", collatz::collatz_length(n1));

    println!("User input:");
    user_input::user_input();
}

fn el_maybe() {
    println!("el_maybe");
    let maybe_number = Some(42);
    if let Some(number) = maybe_number {
        println!("The number is: {:?}", number);
    }
    else {
        println!("There is no number");
    }
}
