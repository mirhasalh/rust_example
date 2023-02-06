pub fn run() {
    let mut i = 0;

    // fizzbuzz challenge
    while i <= 15 {
        if i % 15 == 0 {
            println! {"{} fizzbuzz",i};
        } else if i % 3 == 0 {
            println! {"{} fizz",i};
        } else if i % 5 == 0 {
            println! {"{} buzz",i};
        } else {
            println!("{}", i);
        }

        // Increment
        i += 1;
    }

    // For range loop
    for j in 0..3 {
        println!("{}", j);
    }
}
