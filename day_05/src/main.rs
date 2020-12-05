use day_05;

fn main() {
    let max = day_05::get_max_pass_id("data/boarding_pass.txt");
    println!("Maximal boarding pass number is: {}", max);

    let missing_seat = day_05::get_missing_seat("data/boarding_pass.txt");
    println!("Missing id is: {}", missing_seat);
}
