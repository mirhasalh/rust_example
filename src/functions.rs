pub fn run() {
    greetings("Hi", "Doe");

    let sum = sum_of_two(2, 3);

    println!("SUM: {}", sum);

    let n3: i32 = 5;
    // Closure
    let add_two_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("SUM with closure: {}", add_two_nums(2, 3));
}

fn greetings(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn sum_of_two(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
