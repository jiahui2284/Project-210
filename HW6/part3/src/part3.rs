use std::io;

fn main() {
    println!("Enter a non-negative integer k:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: u32 = input.trim().parse().expect("Not a good number!");

    let mut sum = 0u32;
    for i in 1..=k {
        sum += i.pow(2); 
    }

    println!("The sum of the squares from 1 to {} is: {}", k, sum);
}
