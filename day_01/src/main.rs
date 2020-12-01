use day_01;

fn main() {
    let values = day_01::get_expenses("data/ExpenseReport.txt");
    let pair = day_01::find_pair_with_sum(2020, &values);
    println!("The product is: {}", day_01::product_of_pair(pair));

    let triple = day_01::find_triple_with_sum(2020, &values);
    println!("The product of the triple is: {}", day_01::product_of_triple(triple));
}