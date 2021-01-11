fn main() {
    let filename = "data/timetable.txt";
    let product = day_13::calculate_product(filename);
    println!("The wait and bus id product is: {}", product);

    let departure = day_13::calculate_earliest_departure(filename);
    println!("The earliest departure is: {}", departure);
}