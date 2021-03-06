use day_02;

fn main() {
    let num_valid = day_02::count_valid_entries("data/passwords.txt");
    println!("Nr of valid passwords: {}", num_valid);

    let num_new_valid = day_02::count_new_valid_entries("data/passwords.txt");
    println!("Nr of valid passwords with new rules: {}", num_new_valid);
}