fn main() {
    // Theses are seen as the same string values
    let file_path = "C:\\My Documents\\new\\files";

    // This is the raw string verion
    let file_path = r"C:\My Documents\new\files";
}

fn main() {
    let value: i32 = -15;

    // .abs is the method. The method value should be 15.
    println!("{}", value.abs())
}

fn main() {
    let empty_space = "       my content       ";

    // .trim is the method. The method value should be "my content"
    println!("{}", empty_space.trim());
}

fn main() {
    let value: i32 = -15;

    // .pow is the method. The method value should be 225.
    println!("{}", value.pow(2));
}

fn main() {
    let pi: f64 = 3.1415926535897932384;

    println!("The current value of pi is {pi}");

    // The method is .floor, the method value is 3
    println!("{}", pi.floor());

    // The method is .ceil, the method value is 4
    println!("{}", pi.ceil());

    // The method is .round, the method value is 3
    println!("{}", pi.round());
}

fn main() {
    let pi: f64 = 3.1415926535897932384;

    /* The . specifies how many decimal points to display
    The output of this should be 3.14 */
    println!("The current value of pis is {pi:.2}");

    /* You can also add the input at the end.
    The output of this should be 3.16 due to rounding */
    println!("The current value of pis is {:.4}", pi);
}

fn main() {
    let miles_away = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_u8 = miles_away as u8;

    let miles_away = 100.329032;
    let miles_away_f32 = miles_away as f32;
    let miles_away_int = miles_away as i32;
    println!("{miles_away_int}");
}

fn main() {
    let mut year = 2025;

    year = year + 1;
    println!("The new year is {year}");

    year += 1;
    println!("The new year is {year}");
}

fn main() {
    // This would output 'false'
    println!("{}", !true);

    // This would output 'true'
    println!("{}", !false);
}

fn main() {
    println!("{}", "Coke" == "Pepsi"); //false
    println!("{}", "Coke" != "Pepsi"); //false
    println!("{}", "Coke" == "coke"); //false
    println!("{}", "Coke" == "Coke"); //true

    println!("{}", 13 == 13); //true
    println!("{}", 13 != 13); //false
}

fn main() {
    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

    // This will output the enture seasons array
    println!("{:?}", seasons);
    println!("{seasons:?}");

    // This will display it as a vertical list
    println!("{seasons:#?}");
}

fn main() {
    let employee = ("Anna", 29, "IT");

    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;

    let (name, age, department) = employee;

    println!("Name: {name}, age: {age}, department: {department}");

    dbg!(employee);
}

fn main() {
    // This is upto but does NOT include 31
    let month_days = 1..31;

    // This is upto AND including 31
    let month_days = 1..=31;
}

fn main() {
    let letters = 'b'..'f';

    for letter in letters {
        println! {"{letter}"};
    }
}

fn main() {
    let tuple_name = (element_1, element_2, element_3);

    for element in tuple_name {
        println!("{element}");
    }
}

fn main() {
    let colors = ["Red", "Green", "Yellow"];

    for color in colors {
        println!("{color} is a great color!");
    }

    /* Red is a great color!
    Green is a great color!
    Yellow is a great color! */
}
