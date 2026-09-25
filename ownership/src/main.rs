fn main() {
    let mut trip = start_trip();
    visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    trip.push('.');
    show_intinerary(&trip);
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(destination_a: &mut String) {
    destination_a.push_str("Philadelphia");
}

fn visit_new_york(destination_b: &mut String) {
    destination_b.push_str("New York");
}

fn visit_boston(destination_c: &mut String) {
    destination_c.push_str("Boston");
}

fn show_intinerary(trip: &String) {
    println!("{trip}")
}
