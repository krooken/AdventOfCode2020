fn main() {
    let filename = "data/adapters.txt";
    let product = day_10::get_product(filename);
    println!("The distribution is: {}", product);
}