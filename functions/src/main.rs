fn main() {
    apply_to_jobs(35, "Rust Developer");
    println!("{}", (is_even(10)));
    println!("{}", (is_even(5)));
    println!("{}", (is_odd(5)));
    println!("{}", (is_odd(10)));
    println!("{:?}", alphabets("aardvark"));
    println!("{:?}", alphabets("zoology"));
    println!("{:?}", alphabets("zebra"));
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I am applying to {number} {title} jobs")
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn is_odd(number: i32) -> bool {
    number % 2 != 0
}

fn alphabets(text: &str) -> (bool, bool) {
    let a = text.contains("a");
    let z = text.contains("z");
    (a, z)

    // Can also write this in a single line
    // (text.contains("a"), text.contains("z"))
}
