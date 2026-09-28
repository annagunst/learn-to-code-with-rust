fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");
    let string_reference: &String = &action_hero;
    println!("{string_reference}");

    let first_name: &str = &action_hero[0..6];
    println!("{first_name}");

    let last_name = &action_hero[7..21];
    println!("{last_name}");
}

fn main() {
    let action_hero: &str = "Arnold Schwarzenegger"; // This is renting the entire house
    let string_reference: &str = &action_hero[0..6];
    println!("{string_reference}");

    let first_name = {
        let action_hero = "Arnold Schwarzenegger";
        &action_hero[0..6]
    };

    println!("{first_name}")
}

fn main() {
    let food = "pizza";
    println!("{}", food.len());
    let pizza_slice = &food[0..3];
    println!("{}", pizza_slice.len());

    let pizza_emoji = "🍕"; // emojis are 4 bytes
    println!("{}", pizza_emoji.len());
    let pizza_emoji_slice = &pizza_emoji[0..4]; // anything other than 4 here will cause an error
    println!("{}", pizza_emoji_slice.len());
}

fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");

    let first_name: &str = &action_hero[0..6];
    // Could also write this as &action_hero[..6] since it is starting at 0
    println!("His first name is {first_name}");

    let last_name: &str = &action_hero[7..21];
    // Could also write this as &action_hero[7..] since it is going to the end
    println!("His last name is {last_name}");

    let full_name: &str = &action_hero[..];
    // Could also write this as &action_here since it the entire string
    println!("His full name is {full_name}");
}

fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day")
}
fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");
    do_hero_stuff(&action_hero);
    let another_action_hero: &str = "Sylvester Stallone"; // This is automatically a string slice
    do_hero_stuff(another_action_hero);
}

fn main() {
    let values: [i32; 6] = [4, 8, 15, 16, 23, 42];

    let my_slice: &[i32] = &values[0..4];
    println!("{my_slice:?}");

    let my_slice: &[i32] = &values[2..4];
    println!("{my_slice:?}");

    let my_whole_slice: &[i32] = &values[..];
    println!("{my_whole_slice:?}");
}

fn main() {
    let values: [i32; 6] = [4, 8, 15, 16, 23, 42];

    let regular_reference: &[i32; 6] = &values; // this is not a slice, just a reference
    println!("{regular_reference:?}");

    // let slice_of_three = &values[..3]; // this is an array slice

    print_length(regular_reference);
}

fn print_length(reference: &[i32]) {
    // This is the preferred syntax
    println!("{}", reference.len());
}

fn main() {
    let mut my_array: [i32; 5] = [10, 15, 20, 25, 30];
    let my_slice: &mut [i32] = &mut my_array[2..4]; // This is a mutable reference to an array, a mutable slice

    println!("My slice: {:?}", my_slice);

    my_slice[0] = 100;
    println!("My slice: {:?}", my_slice);
    println!("My array: {:?}", my_array); // 20 will be changed to 100
}
