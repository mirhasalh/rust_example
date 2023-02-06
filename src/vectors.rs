// While array are not resizeable then vectore are resizeable

pub fn run() {
    let mut numbers: Vec<i32> = vec![3, 7, 4, 2, 8];

    println!("{:?}", numbers);

    // Can push since we use vector
    numbers.push(1);

    println!("Specific value: {}", numbers[2]);

    println!("Length: {}", numbers.len());

    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Slice a range of a vector
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);

    // Mutate in loop
    for n in numbers.iter_mut() {
        *n *= 2;
    }

    println!("Mutated vector: {:?}", numbers);
}
