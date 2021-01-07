fn main() {
    let rule_file = "data/rules.txt";
    let tickets_file = "data/nearby_tickets.txt";
    let sum = day_16::count_valid_tickets(rule_file, tickets_file);
    println!("The sum of the invalid tickets is: {}", sum);

    let my_ticket_file = "data/my_ticket.txt";
    let prefix = "departure";
    let product = day_16::multiply_fields(rule_file, tickets_file, my_ticket_file, prefix);
    println!("The product of the departure fields is: {}", product);
}