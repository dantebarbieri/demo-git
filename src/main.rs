fn main() {
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line.");
    if name.is_empty() {
        println!("Not a valid name!");
        return ;
    }
    println!("Hello, {}!", name);
}
