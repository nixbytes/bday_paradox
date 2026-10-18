use rand::Rng;

/// Calculates the probability that at least two people in a group of 'n'
/// have the same birthday (assuming 365 days in a year).
pub fn birthday_paradox_probability(n: usize) -> f64 {
    let mut probability_no_collision = 1.0;
    for i in 0..n {
        probability_no_collision *= (365 - i) as f64 / 365.0;
    }
    1.0 - probability_no_collision
}

/// Simulates the birthday paradox to see how many people are needed
/// on average to have an 50% chance of a collision.
pub fn simulate_birthday_paradox() -> usize {
    let mut rng = rand::thread_rng();
    let mut people_count = 0;
    let mut birthdays = Vec::new();

    loop {
        people_count += 1;
        let birthday = rng.gen_range(1..=365);
        if birthdays.contains(&birthday) {
            return people_count;
        }
        birthdays.push(birthday);
    }
}

fn main() {
    let n = 23;
    let prob = birthday_paradox_probability(n);
    println!("Probability of a birthday collision with {} people: {:.2}%", n, prob * 100.0);

    let first_collision = simulate_birthday_paradox();
    println!("Simulation: First collision occurred with {} people.", first_collision);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probability_at_23() {
        // The classic result is ~50% for 23 people
        let prob = birthday_paradox_probability(23);
        assert!(prob > 0.49 && prob < 0.51);
    }

    #[test]
    fn test_probability_at_1() {
        assert_eq!(birthday_paradox_probability(1), 0.0);
    }

    #[test]
    fn test_probability_at_0() {
        assert_eq!(birthday_paradox_probability(0), 0.0);
    }
}
