#![allow(dead_code)]
// This is necessary because the only thing we do with the Event type is print it out.
// Due to a nuance of how the compiler checks for dead code this causes it to think that the code is unused.
// It can be ignored for the purpose of this exercise.

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    Moving(Direction),
    Waiting(i32),
    CarDoorOpen,
    CarDoorClose,
    LobbyCall(i32, Direction),
    CarCall(i32),
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::Waiting(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::CarDoorOpen
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::CarDoorClose
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::LobbyCall(floor, dir)
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::CarCall(floor)
}

pub fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}