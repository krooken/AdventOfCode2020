fn main() {
    let filename = "data/seats.txt";
    let occupied = day_11::simulate(filename);
    println!("Nr of occupied at fixed point: {}", occupied);
}