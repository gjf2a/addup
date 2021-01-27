use std::env;

fn main() {
    let total = env::args()
        .skip(1)
        .map(|v| v.parse::<f64>().unwrap())
        .sum::<f64>();
    println!("Total: {}", total);
}
