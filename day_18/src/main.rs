fn main() {
    let filename = "data/math.txt";
    let sum = day_18::sum_expressions(filename);
    println!("Sum of all expressions: {}", sum);

    let sum = day_18::sum_expressions_precedence(filename);
    println!("Sum of all expressions with precedence: {}", sum);
}