use std::io;

fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut temp;

    if n == 0 {
        return a;
    }

    for _ in 2..=n {
        temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main() {
    println!("Input a number");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line.");

    let n: u64 = n.trim().parse().expect("Is not a number");

    println!("{n}th number of fibonacci array is: {}", fibonacci(n));
}
