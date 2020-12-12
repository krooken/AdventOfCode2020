fn main() {
    let filename = "data/directions.txt";
    let distance_to_destination = day_12::sail_to_destination(filename);
    println!("Distance to destination: {}", distance_to_destination);

    let distance_with_waypoints = day_12::sail_with_waypoint(filename);
    println!("Distance with waypoints: {}", distance_with_waypoints);
}