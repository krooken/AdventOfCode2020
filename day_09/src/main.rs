fn main() {
    let filename = "data/code.txt";
    let row = day_09::find_invalid(filename, 25);
    println!("First invalid number is: {}", row);

    let sum = day_09::get_min_max_sum(filename, 25);
    println!("Weakness is: {}", sum);
}