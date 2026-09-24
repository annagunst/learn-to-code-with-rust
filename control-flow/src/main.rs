fn color_to_number(color: &str) {
    if color == "red" {
        println!("1")
    } else if color == "green" {
        println!("2")
    } else if color == "blue" {
        println!("3")
    } else {
        println!("0")
    }
}

fn color_to_number(color: &str) {
    match color {
        "red" => println!("1"),
        "green" => println!("2"),
        "blue" => println!("3"),
        _ => println!("0"),
    }
}

fn factorial_iterative(number: i32) -> i32 {
    let mut product = 1;
    let mut count = number;

    while count > 0 {
        product *= count;
        count -= 1;
    }

    product
}

fn factorial_recursive(number: i32) -> i32 {
    // This is the base case
    if number == 1 {
        return 1;
    }

    number * factorial_recursive(number - 1)
}

fn main() {
    color_to_number("red");
    color_to_number("oranage");

    println!("{}", factorial_iterative(5));

    println!("{}", factorial_recursive(5));
}
