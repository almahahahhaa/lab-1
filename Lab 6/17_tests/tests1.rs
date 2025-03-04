fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert() {
        // even number (should return true)
        assert!(is_even(4));
        // odd number (should return false)
        assert!(!is_even(3));
    }
}
