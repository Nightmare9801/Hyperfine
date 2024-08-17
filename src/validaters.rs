use crate::{arithmetic::integer_sqrt, getters::get_divisors};

/// The function `is_mersenne_number` in Rust checks if a given number plus one is a Mersenne number.
/// 
/// Arguments:
/// 
/// * `n`: The `n` parameter in the `is_mersenne_number` function represents a number that will be
/// checked to determine if it is a Mersenne number.
/// 
/// Returns:
/// 
/// The function `is_mersenne_number` returns a boolean value indicating whether the input number `n`
/// (after incrementing by 1) is a Mersenne number.
pub fn is_mersenne_number(mut n: u128) -> bool {
    n += 1;
    (n > 0) && ((n & (n - 1)) == 0)
}

/// The function `is_prime` in Rust checks if a given number is a prime number.
/// 
/// Arguments:
/// 
/// * `n`: The function `is_prime` takes an unsigned 128-bit integer `n` as input and returns a boolean
/// value indicating whether `n` is a prime number or not.
/// 
/// Returns:
/// 
/// The function `is_prime` returns a boolean value indicating whether the input number `n` is a prime
/// number or not.
pub fn is_prime(n: u128) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    for i in 2..integer_sqrt(n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// The function `is_composite` in Rust determines if a given number is composite by checking if it is
/// not prime.
/// 
/// Arguments:
/// 
/// * `n`: u128
/// 
/// Returns:
/// 
/// The function `is_composite` returns a boolean value indicating whether the input number `n` is a
/// composite number. It uses the `is_prime` function to determine if `n` is not a prime number, and
/// then negates the result to determine if `n` is a composite number.
pub fn is_composite(n: u128) -> bool {
    !is_prime(n)
}

/// The function `is_perfect_number` in Rust checks if a given number is a perfect number.
/// 
/// Arguments:
/// 
/// * `n`: The function `is_perfect_number` takes an unsigned 128-bit integer `n` as input and returns a
/// boolean value indicating whether `n` is a perfect number.
/// 
/// Returns:
/// 
/// The function `is_perfect_number` returns a boolean value indicating whether the input number `n` is
/// a perfect number or not.
pub fn is_perfect_number(n: u128) -> bool {
    let divisors: Vec<u128> = get_divisors(n);
    let mut summation: u128 = divisors.iter().sum();
    summation -= n;
    summation == n
}

/// The function `is_deficient_number` in Rust determines if a given number is a deficient number based
/// on its divisors.
/// 
/// Arguments:
/// 
/// * `n`: The function `is_deficient_number` takes an input parameter `n` of type `u128`, which
/// represents a positive integer for which we want to determine if it is a deficient number.
/// 
/// Returns:
/// 
/// The function `is_deficient_number` returns a boolean value indicating whether the input number `n`
/// is a deficient number or not.
pub fn is_deficient_number(n: u128) -> bool {
    let divisors: Vec<u128> = get_divisors(n);
    let mut summation: u128 = divisors.iter().sum();
    summation -= n;
    summation < n
}

/// The function `is_abundant_number` in Rust determines if a given number is an abundant number by
/// checking if the sum of its divisors is greater than the number itself.
/// 
/// Arguments:
/// 
/// * `n`: The function `is_abundant_number` takes an input `n` of type `u128`, which represents a
/// positive integer for which we want to determine if it is an abundant number. An abundant number is a
/// number for which the sum of its proper divisors (excluding itself) is greater
/// 
/// Returns:
/// 
/// The function `is_abundant_number` returns a boolean value indicating whether the input number `n` is
/// an abundant number or not.
pub fn is_abundant_number(n: u128) -> bool {
    let divisors: Vec<u128> = get_divisors(n);
    let mut summation: u128 = divisors.iter().sum();
    summation -= n;
    summation > n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mersenne_test() {
        assert!(is_mersenne_number(127));
        assert!(is_mersenne_number(255));
        assert!(is_mersenne_number(511));
        assert!(!is_mersenne_number(259));
        assert!(!is_mersenne_number(117));
    }

    #[test]
    fn primality_test() {
        assert!(is_prime(2));
        assert!(is_prime(7));
        assert!(is_prime(107));
        assert!(is_prime(7919));
    }

    #[test]
    fn perfect_test() {
        assert!(is_perfect_number(6));
        assert!(!is_perfect_number(9));
        assert!(is_perfect_number(28));
    }

    #[test]
    fn abundant_test() {
        assert!(!is_abundant_number(6));
        assert!(is_abundant_number(12));
        assert!(!is_abundant_number(28));
    }

    #[test]
    fn deficient_test() {
        assert!(!is_deficient_number(6));
        assert!(is_deficient_number(9));
        assert!(is_deficient_number(16));
        assert!(!is_deficient_number(28));
    }

    #[test]
    fn composite_test() {
        assert!(!is_composite(2));
        assert!(!is_composite(7));
        assert!(!is_composite(107));
        assert!(!is_composite(7919));
        assert!(is_composite(28));
        assert!(is_composite(128));
        assert!(is_composite(243));
        assert!(is_composite(2401));
    }
}
