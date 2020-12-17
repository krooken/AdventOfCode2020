fn main() {
    let filename = "data/cubes.txt";
    let nr_cubes = day_17::simulate(&filename);
    println!("Nr of active cubes after 6 cycles: {}", nr_cubes);

    let nr_cubes = day_17::simulate4(&filename);
    println!("Nr of active cubes in 4D after 6 cycles: {}", nr_cubes);
}