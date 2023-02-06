pub fn run() {
    let height = 180;

    if height >= 180 {
        println!("You in");
    } else {
        println!("Try later");
    }

    // Shorthand of condition
    let accept = if height >= 180 { true } else { false };

    if accept {
        println!("You're accepted");
    }
}
