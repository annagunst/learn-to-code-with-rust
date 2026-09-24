fn main() {
    if true {
        println!("This line will be output");
    }

    if false {
        println!("This line will NOT be output");
    }
}

fn main() {
    let season = "summer";

    if season == "summer" {
        println!("School is out");
    } else if season == "winter" {
        println!("So cold");
    } else if season == "fall" {
        println!("Leaves falling");
    } else if season == "spring" {
        println!("Lots of rain");
    }
}

fn main() {
    let evaluation: bool = true;

    match evaluation {
        true => {
            println!("The value is true");
        }
        false => {
            println!("This value is false");
        }
    }
}

fn main() {
    let evaluation: bool = false;

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    println!("{value}");
}

fn main() {
    let season = "winter";

    if season == "summer" {
        println!("School is out");
    } else if season == "winter" {
        println!("So cold");
    } else if season == "fall" {
        println!("Leaves falling");
    } else if season == "spring" {
        println!("Lots of rain");
    }

    // This refactored code is below
    match season {
        "summer" => {
            println!("School is out")
        }

        /* You can remove the {}, println will then be the final return value of the value.
        println will return an empty tuple */
        "winter" => {
            println!("So cold")
        }
    }

    // Can also write it like this
    match season {
        "summer" => println!("School is out"),
        "winter" => println!("So cold"),
        _ => println!("Lots of rain"),
    }
}

fn main() {
    let number = 8;

    match number {
        // "value" is a banana
        value if value % 2 == 0 => println!("{value} is an even number"),
        value if value % 2 != 0 => println!("{value} is an odd number"),
        // _ => println!("Unknown"),
        _ => unreachable!(),
    }
}

fn main() {
    let mut seconds = 21;

    loop {
        if seconds <= 0 {
            println!("Blastoff!");
            /* This breaks the loop
            Must use the ‘break’ keyword or it could still create an infinite loop
            This loop would infinitely print "Blastoff" */
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds..");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blast off..");
        seconds -= 1;
    }
}

fn main() {
    let mut seconds = 21;

    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds..");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds to blast off..");
        seconds -= 1;
    }
}
fn countdown(seconds: i32) {
    // This is the base case
    if seconds == 0 {
        println!("Blastoff!");
    } else {
        println!("{seconds} seconds to blastoff..");
        // This is the recursion
        countdown(seconds - 1);
    }
}

fn main() {
    countdown(5)
}
