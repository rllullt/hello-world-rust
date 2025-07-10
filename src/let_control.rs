use std::time::Duration;

fn sleep_for(secs: f32) {
    let result = Duration::try_from_secs_f32(secs);

    if let Ok(duration) = result {
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    }
}

/// hex_or_die using let if {} else {} expressions
// fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
//     let s = if let Some(s) = maybe_string {
//         s
//     } else {
//         return Err(String::from("got None"));
//     };

//     let first_byte_char = if let Some(first) = s.chars().next() {
//         first
//     } else {
//         return Err(String::from("got empty string"));
//     };

//     let digit = if let Some(digit) = first_byte_char.to_digit(16) {
//         digit
//     } else {
//         return Err(String::from("not a hex digit"));
//     };

//     Ok(digit)
// }

/// hex_or_die using let else expressions
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let Some(s) = maybe_string else {
        return Err(String::from("got None"));
    };

    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };

    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };

    Ok(digit)
}


pub fn main() {
    sleep_for(-10.0);
    sleep_for(0.8);

    let mut name = String::from("Compr. Rust 🦀");
    while let Some(c) = name.pop() {
        print!("{}", c);
    }
    println!();
    // (There are more efficient ways to reverse a string!)

    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}
