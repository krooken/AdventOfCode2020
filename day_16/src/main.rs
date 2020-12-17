fn main() {
    let rule_file = "data/rules.txt";
    let tickets_file = "data/nearby_tickets.txt";
    let sum = day_16::count_valid_tickets(rule_file, tickets_file);
    println!("The sum of the invalid tickets is: {}", sum);
}