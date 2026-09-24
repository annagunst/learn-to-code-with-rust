fn main() {
    /* This will output "Opening my pizza store in Brooklyn"
    "Brooklyn" was passed in as the argument for the neighborhood parameter */
    open_store("Brooklyn");

    /* This will output "Opening my pizza store in Queens"
    "Queens" was passed in as the argument for the neighborhood parameter */
    open_store("Queens");

    // This will output "Baking a pizza"
    bake_pizza(1, "cheese");

    // This will output "So much $$$, so little time"
    swim_in_profit();
    swim_in_profit();
    swim_in_profit();
}

fn open_store(neighbordhood: &str) {
    println!("Opening my pizza store in {neighbordhood}");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizza");
}

fn swim_in_profit() {
    println!("So much $$$, so little time");
}

fn main() {
    let result = square(5);
    println!("The square of 5 is {result}");

    let result = square(13);
    println!("The square of 13 is {result}");
}

fn square(number: i32) -> i32 {
    return number * number;
    // Any code after this point will NOT run
}

fn main() {
    let result = square(5);
    println!("The square of 5 is {result}");

    let result = square(13);
    println!("The square of 13 is {result}");
}

fn square(number: i32) -> i32 {
    number * number
}

fn main() {
    let result = mystery();
}

fn mystery() {
    println!("Hello there")
}

fn main() {
    let multiplier = 3;

    let calculation = {
        // This is an isolated scope, it is an independent execution environment that is nested within the 'main' function
        let value = 5 + 4;
        value * multiplier
    };

    /* This will output 27.
    value = 9., multiple = 3, calculation = 9 * 3  */
    println!("{calculation}");
}
