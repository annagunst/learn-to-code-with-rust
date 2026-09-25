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

fn main() {
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);
}

fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    // meal is a reference string
    println!("Meal steps: {meal}");
    // meal goes out of scope here
}

fn main() {
    let car: String = String::from("Red"); // car remains the owner the whole time

    let ref1: &String = &car; // this is an immutable reference because mut keyword was not used
    let ref2: &String = &car; // this is also an immutable reference because mut keyword was not used

    println!("{ref1} and {ref2} and {}", &car); //none of the borrows involve the mut keyword, so they all reuse the same data on the haep
}

fn main() {
    let mut car: String = String::from("Red"); // car remains the owner

    let ref1: &mut String = &mut car;
    /*
    ref_1 borrows the "Red" string and has permission to udpate it.
    This works because ref1 is not used after this line, thus no potential to change the original string
    ref1 lifetime ends here
    */
    let ref2: &String = &car;

    println!("{ref2}"); //none of the borrows involve the mut keyword, so they all reuse the same data on the heap

    // ref1 scope ends here
}

// Ownership with Immutable and Mutable References
fn main() {
    let coffee = String::from("Mocha");
    let a = &coffee; // this is an immutable reference
    let b = a; // this is also an immutable reference to coffee

    println!("{a} and {b}");
}

fn main() {
    let mut coffee: String = String::from("Mocha"); // Coffee is the original owner of the "Mocha" string

    let a: &mut String = &mut coffee; // a is the owner of the mutable reference

    println!("{a}"); // This is valid at this point, a is still the owner of the mutable reference

    let b: &mut String = a; // ownership of the mutable reference is moved to b, this invalidates a

    println!("{a} and {b}"); // This does NOT work because a is no longer valid
}

// Dangling References
// EXAMPLE OF A DANGLING REFERENCE VIOLATION
fn main() {
    let reference: &String = create_city(); // This is referencing a string that no longer exists
}

fn create_city() -> &String {
    let city: String = String::from("New York"); // city is the owner of the "New York" string, it is responsible for deallocation
    &city // This is the dangling reference, it referring to a string that will go out of scope and is cleared off the heap
} // city scope ends here, "New York" is deallocated
// This would work if you used String instead of &String

fn main() {
    let registrations: [bool; 3] = [true, false, true]; // registration is the owner of the 3 booleans 

    let first: bool = registrations[0]; // Rust creates a full copy and assigns it to the first variable 

    println!("{first} and {registrations:?}"); // both are valid, an ownership move is NOT done 

    let languages: [String; 2] = [String::from("Rust"), String::from("Javascript")]; // languages owns the array, the array owns the values 

    let first: &String = &languages[0].clone(); // borrow a refernce to just the first element 

    println!("{first} and {languages:?}");
}

fn main() {
    let registrations: (bool, bool, bool) = (true, false, true); // registration is the owner of the 3 booleans
    let first: bool = registrations.0; // Rust creates a full copy and assigns it to the first variable
    println!("{first} and {registrations:?}"); // both are valid, an ownership move is NOT done

    let languages: (String, String) = (String::from("Rust"), String::from("Javascript")); // languages owns the array, the array owns the values
    let first: &String = &languages.0; // borrow a refernce to just the first element
    println!("{first} and {languages:?}");
}
