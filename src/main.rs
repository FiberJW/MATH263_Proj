use std::time::Instant;

fn sum_of_floats(f: fn(i32) -> f64, number_of_iterations: i32, starting_number: f64) -> f64 {
    let mut sum = starting_number;
    for i in 0..number_of_iterations {
        sum = sum + f(i);
    }
    sum
}

fn gen_pi(number_of_iterations: i32) -> f64 {
    let now = Instant::now();

    fn gen_term(i: i32) -> f64 {
        (-1f64).powi(i) / (2 * i + 1) as f64
    }

    let estimate = sum_of_floats(gen_term, number_of_iterations, 0.0) * 4.0;
    println!("Time used to estimate PI using {} iterations of the Rabinowitz and Wagon's spigot algorithm: {:.3} seconds", number_of_iterations, now.elapsed().as_secs() as f64);
    estimate
}

fn main() {
    let pi_of_ten_million_iterations = gen_pi(10_000_000);
    let pi_of_100_million_iterations = gen_pi(100_000_000);
    let pi_of_one_billion_iterations = gen_pi(1_000_000_000);
    let system_pi = std::f64::consts::PI;

    println!("system pi constant: {}", system_pi);
    println!(
        "pi estimate of 10M iterations: {}",
        pi_of_ten_million_iterations
    );
    println!(
        "pi estimate of 100M iterations: {}",
        pi_of_100_million_iterations
    );
    println!(
        "pi estimate of 1B iterations: {}",
        pi_of_one_billion_iterations
    );
}
