use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut c: u64 = 0;
    let n: u64 = args[1].parse::<u64>().unwrap();

    // Fibonacci
    if n < 0 {
        println!("Incorrect input");
    }
    if n == 0 {
        println!("{:?}", a);
    }
    if n == 1 {
        println!("{:?}", b);
    }
    if n > 94 {
        println!("Can't calculate numbers bigger than 94");
    }
    else {
        for i in 2..n {
            c = a + b;
            a = b;
            b = c;
        }
        println!("{:?}", b);
    }

}
