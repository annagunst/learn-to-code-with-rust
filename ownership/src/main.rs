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
