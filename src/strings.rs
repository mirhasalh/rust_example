pub fn run() {
    // Use String::from to enable string methods
    // Put mut to make it mutable
    let mut hello = String::from("Hello ");

    // Use push to add single char
    hello.push('W');

    // Use push_str to add multiple char
    hello.push_str("orld!");

    println!("{}", hello);

    // There's a lot of methods, here's the example
    println!("Is contain a word 'Hello': {}", hello.contains("Hello"));

    // Loop through string
    for e in hello.split_whitespace() {
        println!("{}", e);
    }
}
