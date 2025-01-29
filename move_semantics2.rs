fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // Borrow vec0 while passing it to fill_vec
        let vec1 = fill_vec(vec0.clone());  // Clone vec0 to keep a copy of it

        assert_eq!(vec0, [22, 44, 66]);  // vec0 remains unchanged
        assert_eq!(vec1, [22, 44, 66, 88]);  // vec1 has 88 added
    }
}