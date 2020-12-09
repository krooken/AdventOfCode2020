fn main() {
    let filename = "data/code.txt";
    let row = day_09::find_invalid(filename, 25);
    println!("First invalid row is: {}", row);
}