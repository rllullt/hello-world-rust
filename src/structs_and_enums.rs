struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Newtons(f64);
impl Newtons {
    fn add(a: Newtons, b: Newtons) {
        todo!();
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

use std::mem::transmute;

macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
    };
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, (*person).age);
}

pub fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("Describe function receives a Person shared reference (&Person)");
    println!("struct Person parameters are accessed via ‘.’");
    println!("Inside fn describe, person.name and (*person).name are equivalent (Rust dereferences automatically)");
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);

    // Add f64 to newtype Newtons cannot happen due to lack of `add` function implementation
    println!("Newtons can be debug printed thanks to implmenting the trait `Debug` by using #[derive(Debug)]");
    let n = Newtons(10.0);
    println!("The force is {n:?} N");

    let dir = Direction::Left;
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: {player_move:?}");

    println!("Bitwise representation of enums (may be different in every program compilation/running). UNSAFE");
    unsafe {
        println!("bool:");
        dbg_bits!(false, u8);
        dbg_bits!(true, u8);

        println!("Option<bool>:");
        dbg_bits!(None::<bool>, u8);
        dbg_bits!(Some(false), u8);
        dbg_bits!(Some(true), u8);

        println!("Option<Option<bool>>:");
        dbg_bits!(Some(Some(false)), u8);
        dbg_bits!(Some(Some(true)), u8);
        dbg_bits!(Some(None::<bool>), u8);
        dbg_bits!(None::<Option<bool>>, u8);

        println!("Option<&i32>:");
        dbg_bits!(None::<&i32>, usize);
        dbg_bits!(Some(&0i32), usize);
    }
}