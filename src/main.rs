mod user_input;
mod fibo;
mod factorial;
mod collatz;
mod arrays;
mod geometry;
mod structs_and_enums;
mod elevator_events;
mod let_control;
mod expression_evaluation;

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

    println!("Arrays:");
    arrays::an_array();

    let tuple = (1, 5, 3);
    println!(
        "{tuple:?}: {}",
        if check_order(tuple) { "ordered" } else { "unordered" }
    );

    println!("Transpose a 3x3 matrix:");
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    // dbg!(matrix);
    let transposed = arrays::transpose(matrix);
    // dbg!(transposed);
    assert_eq!(transposed, [[101, 201, 301], [102, 202, 302], [103, 203, 303]]);
    println!("All ok if this prints");

    println!("Play with a shared reference:");
    let mut a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    dbg!(a);
    dbg!(b);
    dbg!(r);
    println!("At this point, a cannot be reasigned (even though it is mutable) because it was already ‘borrowed’ by r");
    // a = 'C';  // r is still a reference to a, so it will change
    r = &b;
    dbg!(r);
    println!("Now r is a reference to b, so a can be reasigned");
    a = 'C';  // r is now a reference to b, so it will not change
    dbg!(a);
    dbg!(b);
    dbg!(r);
    dbg!(r.is_ascii());

    println!("Play with a mutable (exclusive) reference:");
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
    point.0 = 10;
    println!("point: {point:?}");

    println!("Check a slice!");
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");
    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");

    println!("Playing with both string types of Rust:");
    let s1: &str = "World";
    println!("s1: {s1}");
    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
    let s3: &str = &s2[2..9];
    println!("s3: {s3}");

    println!("More on strings, with bytes:");
    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]);

    println!("Raw strings:");
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");

    println!("Testing geometry:");
    geometry::main();

    println!("Testing structs:");
    structs_and_enums::main();

    elevator_events::main();

    println!("Let control flow:");
    let_control::main();

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

fn check_order(tuple: (i32, i32, i32)) -> bool {
    println!("check_order of tuple with pattern matching");
    let (left, middle, right) = tuple;
    left < middle && middle < right
}
