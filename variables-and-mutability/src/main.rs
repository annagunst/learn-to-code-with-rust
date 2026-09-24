use std::{char::decode_utf16, string, thread::Scope};

const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "summer";

    let mut points_scored: i32 = 28;

    let points_scored = 35;

    let event_time = "06:00";

    let event_time = 6;

    // #[allow(unused_variables)]
    let _favorite_beverage = "lemonade";

    println!("The end of {season} is the perfect time to kick off football season");
    println!("The game started at {event_time}.");
    println!(
        "The winning team scored {points_scored} points, {TOUCHDOWN_POINTS} of which were from a touchdown!"
    );
}
