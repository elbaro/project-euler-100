extern crate primal;

fn main() {
    println!("{}", primal::StreamingSieve::nth_prime(10_001));
}
