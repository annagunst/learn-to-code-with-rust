fn main() {
    let variable_1 = 1;
    // variable_1 is now in scope

    let variable_2 = 2;
    // variable_2 is now in scope

    println!("{variable_1}");
    println!("{variable_2}");
} // variable_2 goes out of scope, then variable_1 goes out of scope

fn main() {
    let time = 2025;
    let year = time;

    // This would print "The time is 2025. It is 2025"
    println!("The time is {time}. It is the year {year}");
}

fn main() {
    let food = "pasta";
    String::new();

    let text = String::new();

    let candy = String::from("KitKat");
}

fn main() {
    let mut name = String::from("Boris");

    /* Stack will store reference, length, and capacity
    Heap will store "Boris"
    */

    // This will print "Borris"
    println!("{name}");

    name.push_str("Pask");

    // This will print "Borris Paskl"
    println!("{name}");
}

fn main() {
    let person = String::from("Boris");

    println!("My name is {person}");
    // person goes out of scope after this line

    let genius = person; // The move occurs here
    // genius is responsible for deallocation

    // This would lead to a 'borrow of moved value' error
    println!("My name is {person}");
}

fn main() {
    let person = String::from("Boris");
    let genius = person.clone();

    println!("This is {person}");
}

fn main() {
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
}

fn main() {
    let my_stack_value = 2;
    let my_stack_reference = &my_stack_value; // This will print "2"
    println!("{}", *my_stack_reference);

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
    println!("{}", *my_heap_reference); // This will print "Toyota"
}

fn main() {
    /*
    String- dynamic piece of text stored on the heap at run time

    &String ("reference String")- reference to a heap string

    str- hardcoded, read-only piece of text, binary

    &str ("reference str")- reference to the text in the memory that has loaded the binary file
    */

    let ice_cream: &str = "Cookies and cream"; // This is referencing the memory
    println!("{}", ice_cream);
}

fn main() {
    let ice_cream: &str = "Cookies and cream"; // This is referencing the text from the actual binary
    let dessert: &str = ice_cream; // This is also referencing the text from the binary

    println!("{ice_cream} {dessert}")
}

fn main() {
    let apples: i32 = 6;
    print_my_value(apples); // let values = apples;

    println!("{apples} is still valid");

    fn print_my_value(value: i32) {
        println!("Your value is {value}")
    };
}
fn main() {
    let mut burger: String = String::from("Burger");
    add_fries(burger); // let meal = burger;
}

fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}")
}

fn main() {
    /* Ownership of "Chocolate Mousse" is moved from the cake variable in the bake_cake function
    to the cake variable in the main function */
    println!("I now have a {cake} cake");
    let cake = bake_cake();
}

fn bake_cake() -> String {
    let cake = String::from("Chocolate Mousse");

    // The return statement allows the return value to live on even though the function ends
    return cake;

    /* Could also write it as below
    String::from("Chocolate Mousse")
     */
}
