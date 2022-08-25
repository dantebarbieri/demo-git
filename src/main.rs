fn main() {
    let name: &str = "Dante Barbieri";
    println!("Hello, {}!", name);
    let names: Vec<&str> = name.split_whitespace().collect();
    println!("First: {}\nLast: {}", names[0], names[1]);
}
