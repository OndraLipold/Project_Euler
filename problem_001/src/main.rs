/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3,5,6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

fn main() {
    println!("{}", sum_multiples(1000));
}

fn sum_multiples(target: usize) -> usize {
    let mut sum: usize = 0;

    for i in 1..target {
        if i % 3 == 0 {
            sum += i;
        } else if i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiples_10() {
        assert_eq!(sum_multiples(10), 23);
    }
}
