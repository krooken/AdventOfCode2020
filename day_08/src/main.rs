fn main() {
    let acc = day_08::get_acc_of_program("data/boot_code.txt");
    println!("Acc at loop: {}", acc);

    let acc = day_08::get_acc_at_end("data/boot_code.txt");
    println!("Acc at end of changed program: {}", acc);
}