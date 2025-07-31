mod days;

fn main() {
    days::first::hello();
    print!("a + b = {}", days::second::add(3, 5));
}
