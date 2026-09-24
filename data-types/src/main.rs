fn main() {
    let number: i32 = 1_3_3_7;

    let numer_as_i16 = number as i16;

    let float = 1.234567890;

    println!("{float:.3}");

    let with_milk = true;
    let with_sugar = true;

    let is_my_type_of_coffee = (with_milk && with_sugar);

    let is_acceptable_coffee = (with_milk || with_sugar);

    let integers: [i8; 4] = [1, 2, 3, 4];
    dbg!(integers);

    let tuple = (1, 2.1, true, integers);
    dbg!(tuple);
}
