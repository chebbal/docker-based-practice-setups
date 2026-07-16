use hellolib;
use rand::RngExt;

fn test_rng() {
    println!("Example - test random number generator");
    let mut rng = rand::rng();
    let n: u32 = rng.random_range(1..=100);
    println!("Random number(1-100): {n}");

    let b: bool = rng.random();
    println!("Random bool: {b}");

    let f: f64 = rng.random();
    println!("Random float: {f:.4}");

}

fn main() {
    println!("Hello, world! {}", hellolib::add(40,2));
    println!("{}","-".repeat(20));
    test_rng();
    println!("{}","-".repeat(20));
}
