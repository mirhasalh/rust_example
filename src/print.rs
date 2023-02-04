pub fn run() {
    // Print to console
    println! {"Hello World from print.rs"};

    // Formatting
    println! {"{} is {}","Apple","red"};

    // Positional arguments
    println!("{0} is {1} and {0} is {2}", "Apple", "fruit", "red");

    // Named arguments
    println!(
        "{name} likes to write {lang} code",
        name = "Doe",
        lang = "Rust"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Math
    println!("10 + 10 = {}", 10 + 10);
}
