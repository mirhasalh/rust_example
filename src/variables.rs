pub fn run() {
    let name = "Doe";
    let mut age = 29;

    println!("My name is {}, I am {} years old", name, age);
    age = 30;
    println!("My name is {}, I am {} years old", name, age);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    // One line variables
    let (fruit, weight) = ("Apple", 1);
    println!("{} weight is {} kg", fruit, weight);
}
