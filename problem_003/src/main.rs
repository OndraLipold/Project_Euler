/*
 *  The prime factors of 13195 are 5,7,13 and 29.
 *
 *  What is the largest prime factor of the number 600851475143?
 */

fn main() {
    println!("{}", largest_prime_factor(600851475143));
}

fn largest_prime_factor(mut number: usize) -> usize {
    let mut max: usize = 0;
    let mut current = 1;

    while number > 1 || max < number {
        if is_prime(current) {
            if number % current == 0 {
                max = current;
                number = number / current;
            }
        }
        current += 1;
    }
    max
}

fn is_prime(number: usize) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_larger_prime_factor() {
        assert_eq!(largest_prime_factor(317584931803), 3919);
    }

    #[test]
    fn test_small_prime_factor() {
        assert_eq!(largest_prime_factor(13195), 29);
    }
}
