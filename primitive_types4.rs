fn main() {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // Getting a slice from index 1 to 4 (excluding 4)
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}