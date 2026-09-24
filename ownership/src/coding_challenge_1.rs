/*
Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

Repeat the previous example but use a heap String instead.
Will Rust move ownership? Explain why the result is different
from the previous operation.

The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.

In the `main` function, invoke the `eat_meal` function and pass
in your "Salmon" String. Explain what happens when the eat_meal
function runs. Describe the complete movement of ownership of
the "Salmon" String throughout the program.

Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/

fn main() {
    /*
    Declare a `is_concert` variable set to a boolean.
    Declare a `is_event` variable assigned to `is_concert`.
    Will Rust move ownership? State your answer, then confirm
    by trying to printing both variables out.
    */
    let is_concert: bool = true;
    let is_event: bool = is_concert; // Ownership is NOT moved to is_event. Boolean values are copied, not moved

    println!("{}", is_concert);
    println!("{}", is_event);

    /*
    Declare a `sushi` variable to set to a string literal of "Salmon"
    Declare a `dinner` variable assigned to the `sushi` variable.
    Will Rust move ownership? State your answer, then confirm
    by trying to printing both variables out.
    */
    let sushi: &str = "Salmon";
    let dinner: &str = sushi; // Ownership is NOT moved to dinner. String slices are copied, not moved

    println!("{}", sushi);
    println!("{}", dinner);

    /*
    Repeat the previous example but use a heap String instead.
    Will Rust move ownership? Explain why the result is different
    from the previous operation. */
    let sushi_2: String = String::from("Salmon");
    let dinner_2: String = sushi_2; // Ownership of "Salmon" is moved to dinner_2, Strings are moved, not copied

    // println!("{}", sushi_2); // This will not work because after the move, sushi_2 does not exist
    println!("{}", dinner_2);

    /*
    In the `main` function, invoke the `eat_meal` function and pass
    in your "Salmon" String. Explain what happens when the eat_meal
    function runs. Describe the complete movement of ownership of
    the "Salmon" String throughout the program.
    */

    eat_meal(dinner_2); // You have to use dinner_2 because ownership of "Salmon" was transferred to dinner_2 above
}

/*
The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.
*/
/*
Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/
fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}
