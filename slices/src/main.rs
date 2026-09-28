fn main() {
    let mut cereals: [String; 5] = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two: &[String] = &cereals[0..2];
    println!("My first 2 cereals: {first_two:?}");

    let mid_three: &[String] = &cereals[1..4];
    println!("My middle 3 cereals: {mid_three:?}");

    let last_three: &mut [String] = &mut cereals[2..];
    println!("My last 3 cereals: {last_three:?}");

    last_three[2] = String::from("Lucky Charms");

    println!("All my cereals: {cereals:?}");

    let cookie_crisp: &String = &cereals[0];
    let cookie: &str = &cookie_crisp[..6];
    println!("{cookie}");

    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];
    println!("{puffs}");
}
