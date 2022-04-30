fn attacker_success_prob(q: f64, z: f64) -> f64 {
    // q: probability the attacker finds the next block
    // z: block deficit

    let p = 1.0 - q;
    let lambda = z * (q / p);
    let mut sum = 1.0;

    for k in 0..((z + 1.0) as i32) {
        let mut poisson = (-lambda).exp();
        for i in 1..(k + 1) {
            poisson *= lambda / (i as f64);
        }
        let qp = q / p;
        sum -= poisson * (1.0 - qp.powf(z - (k as f64)));
    }

    return sum;
}

fn main() {
    println!("Attack Success Probability: {}", attacker_success_prob(0.1, 1.0));
}
