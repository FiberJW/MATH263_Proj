use std::f64::consts::PI;
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
    println!("\n");

    println!("Estimating PI using Rabinowitz and Wagon's spigot algorithm");

    println!("=================================");
    let pi_of_ten_million_iterations = gen_pi(10_000_000);
    let pi_of_100_million_iterations = gen_pi(100_000_000);
    let pi_of_one_billion_iterations = gen_pi(1_000_000_000);

    println!("=================================");

    println!("system pi constant: {}", PI);

    println!("=================================");

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

    let estimated_pi_string = pi_of_one_billion_iterations.to_string();
    let system_pi_string = PI.to_string();
    let mut correct_digits = 0;

    for (i, c) in system_pi_string.chars().enumerate() {
        if c == estimated_pi_string.chars().nth(i).unwrap() {
            if c != '.' {
                correct_digits += 1;
            }
        } else {
            break;
        }
    }

    println!("=================================");

    println!(
        "Amount of correct digits of PI estimated using 1B iterations of the algorithm: {}",
        correct_digits
    );
}
