pub fn run() {
    // i32 by default
    let x = 2;

    // f64 by default
    let y = 2.5;

    // Declare specific type
    let z: i64 = 45435234344423;

    println!("{},{},{}", x, y, z);

    // Find max
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let is_active: bool = true;

    // Bool from expression of condition
    let is_greater: bool = 10 > 5;

    // Char
    let a_word = 'A';

    let favourite = '\u{2764}';

    println!("{:?}", (x, y, x, is_active, is_greater, a_word, favourite))
}
