/// The `integer_sqrt` function in Rust calculates the integer square root of a given unsigned 128-bit
/// integer.
/// 
/// Arguments:
/// 
/// * `n`: The function `integer_sqrt` takes an unsigned 128-bit integer `n` as input and calculates the
/// integer square root of that number.
/// 
/// Returns:
/// 
/// The function `integer_sqrt` returns the integer part of the square root of the input `n` as a `u128`
/// value.
pub fn integer_sqrt(n: u128) -> u128 {
    if n < 2 {
        return n; // Return n directly for 0 and 1
    }
    
    let mut x = n / 2; // Initial guess
    loop {
        let y = (x + n / x) / 2; // Integer division
        if y >= x {
            return x; // Return the integer part of the square root
        }
        x = y; // Update for the next iteration
    }
}

pub fn inverse_sqrt(n: f64) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_sqrt_test() {
        assert!(integer_sqrt(4) == 2);
        assert!(integer_sqrt(9) == 3);
        assert!(integer_sqrt(23) == 4);
    }
}