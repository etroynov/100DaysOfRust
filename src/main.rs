mod days;

fn main() {
    days::first::hello();
    println!("a + b = {}", days::second::add(3, 5));
    println!("5 years is {} days", days::third::age_to_day(5));

    match days::fourth::read_dir() {
        Ok(text) => println!("Done"),
        Err(e) => println("Cant read current folder {}"),
    }
}
