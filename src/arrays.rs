pub fn run() {
    let numbers: [i32; 5] = [3, 7, 4, 2, 8];

    println!("{:?}", numbers);

    println!("Specific value: {}", numbers[2]);

    println!("Length: {}", numbers.len());

    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Slice a range of an array
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice)
}
