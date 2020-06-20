use primal;

fn main() {
    println!("{}", {
        let sum: usize = primal::Primes::all().take_while(|&x| x < 2_000_000).sum();
        sum
    });
}
