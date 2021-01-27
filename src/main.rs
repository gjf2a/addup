use std::env;

fn main() {
    let mut total = 0.0;
    for arg in env::args().skip(1) {
        match arg.parse::<f64>() {
            Ok(value) => total += value,
            Err(_) => println!("\"{}\" is not parseable", arg)
        }
    }
    println!("Total: {}", total);
}
