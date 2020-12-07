fn main() {
    let num = day_07::count_bags_carrying_bag("data/bag_rules.txt", "shiny gold");
    println!("Number of colors which can carry the bag: {}", num);
}