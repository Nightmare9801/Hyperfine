/// The function `get_divisors` in Rust returns a vector containing all the divisors of a given number.
/// 
/// Arguments:
/// 
/// * `n`: u128
/// 
/// Returns:
/// 
/// The function `get_divisors` returns a vector of all the divisors of the input number `n`.
pub fn get_divisors(n: u128) -> Vec<u128> {
    let mut divisors: Vec<u128> = Vec::new();
    for i in 1..(n + 1) {
        if n % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

#[cfg(test)]
mod tests {
    use super::*;

    
}