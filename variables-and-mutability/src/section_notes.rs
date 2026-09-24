// use std::{char::decode_utf16, thread::Scope};

// fn main() {
//     let apples = 50;
//     let oranges = 14 + 6;
//     let _fruits = apples + oranges;

//     // These are the same
//     // Output will be: This year, my garden has 50 apples
//     println!("This year, my garden has {} apples", apples);
//     println!("This year, my garden has {apples} apples");

//     // These are the same
//     // Output will be: This year, my garden has 50 apples and 20 oranges
//     println!(
//         "This year, my garden has {} apples and {} oranges",
//         apples, oranges
//     );
//     println!("This year my garden has {apples} apples and {oranges} oranges");

//     // These are the same
//     // Output will be: This year, my garden has 50 apples and 20 oranges. I cannot believe I have 50 apples!
//     println!(
//         "This year, my garden has {} apples and {} oranges. I cannot believe I have {} apples!",
//         apples, oranges, apples
//     );
//     println!(
//         "This year, my garden has {0} apples and {1} oranges. I cannot believe I have {0} apples!",
//         apples, oranges
//     );
// }

// fn main() {
//     // Outer scope begin
//     let mut coffee_price = 5.99;

//     // This would print "The price of coffee is 5.99"
//     println!("The price of coffee is {coffee_price}");

//     // This is variable shadowing
//     coffee_price = 6.99;

//     // Outer scope end

//     {
//         // Inner scope begin

//         /* This would print "The price of coffee is 6.99" because it can use outer scope variables.
//         It is using the most recent definition of the variable coffee_price. */
//         println!("The price of coffee is {coffee_price}");

//         /* This is NOT variable shadowing, coffee_price below is seen as a new, independent variable.
//         It has no relation to the coffee_price in the outer scope.
//         This is seen as a new, independent variable. It has no relation to the coffee_price in the outer scope.
//         From this point on, within this inner scope, coffee_price references the variable below. */
//         let coffee_price = 7.99;

//         // This would print "The price of coffee is 7.99" because it is using the most recent definition of the variable coffee_price
//         println!("The price of coffee is {coffee_price}");

//         // Inner scope end
//     }

//     // Outer scope begin

//     // This would print "The price of cofee is 6.99"
//     println!("The price of coffee is {coffee_price}");

//     // Outer scope end
// }

// type Meters = i32; // You are assigning a type, NOT a value. Everywhere in this code, Meters will represent an integer
// fn main() {
//     #[allow(unused_variables)]
//     // This is assumed to be an integer
//     let mile_race_length = 1600;

//     #[allow(unused_variables)]
//     // You can also specify the type manually
//     let mile_race_length: i32 = 1600;

//     // You can also refer to the type defined above
//     let mile_race_length: Meters = 1600;

//     // Types can also be resused
//     let two_mile_race_length: Meters = 3200;

//     println!(
//         "A one mile race is {mile_race_length} meters long and a two mile race is {two_mile_race_length} meters long."
//     );
// }
