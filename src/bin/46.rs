use primal;

fn main() {
    println!(
        "{}",
        (3..)
            .step_by(2)
            .filter(|&i| {
                !primal::Primes::all()
                    .skip(1)
                    .take_while(|&p| p <= i)
                    .any(|p| {
                        let sq = (i - p) / 2;
                        let x = (sq as f32).sqrt() as usize;
                        x * x == sq
                    })
            })
            .next()
            .unwrap()
    );
}
