fn main() {
    let valid_count = day_04::nr_valid_passports("data/passports.txt");
    println!("Nr of valid passports: {}", valid_count);

    let strict_valid_count = day_04::nr_strict_valid_passports("data/passports.txt");
    println!("Nr of strict valid passports: {}", strict_valid_count);
}