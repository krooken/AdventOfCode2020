use day_03;

fn main() {
    let count = day_03::count_trees("data/woods.txt");
    println!("Number of trees in path: {}", count);

    let product = day_03::multiply_trees("data/woods.txt");
    println!("Product of trees in paths: {}", product);
}