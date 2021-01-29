use std::env;

#[derive(Debug,Copy,Clone)]
enum Op {
    Add, Mult
}

impl Op {
    fn from_arg(arg: &str) -> Option<Self> {
        match arg {
            "-m" => Some(Self::Mult),
            "-a" => Some(Self::Add),
            _ => None
        }
    }

    fn identity(&self) -> isize {
        match self {
            Op::Add => 0,
            Op::Mult => 1
        }
    }

    fn apply(&self, a: isize, b: isize) -> isize {
        match self {
            Op::Add => a + b,
            Op::Mult => a * b
        }
    }
}

fn main() {
    let mut operation = Op::Add;
    let mut total = operation.identity();
    for arg in env::args().skip(1) {
        match Op::from_arg(arg.as_str()) {
            Some(op) => {
                operation = op;
                total = operation.identity();
                println!("{:?}", operation);
            }
            None => match arg.parse::<isize>() {
                Ok(value) => total = operation.apply(total, value),
                Err(_) => println!("'{}' is not parseable", arg)
            }
        }
    }
    println!("Total: {}", total);
}