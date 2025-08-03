pub fn read_from_std() {
    println!("Enter your age");

    let mut choice = String::new();

    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    println!("Your age is {}", choice);
}
