fn main() {
    let count = day_06::count_flight_answers("data/answers.txt");
    println!("Number of answers for flight: {}", count);

    let count = day_06::count_flight_answers_all("data/answers.txt");
    println!("Number of answers where all answered yes: {}", count);
}