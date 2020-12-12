fn main() {
    let filename = "data/directions.txt";
    let distance_to_destination = day_12::sail_to_destination(filename);
    println!("Distance to destination: {}", distance_to_destination);
}