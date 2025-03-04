use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // Only insert if the fruit is not already in the basket
        if !basket.contains_key(&fruit) {
            basket.insert(fruit, 1);
        }
    }
}

fn main() {
    let mut basket = HashMap::new();
    // Initial basket with given fruits
    basket.insert(Fruit::Apple, 4);
    basket.insert(Fruit::Mango, 2);
    basket.insert(Fruit::Lychee, 5);
    
    fruit_basket(&mut basket);
    println!("{:?}", basket);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Don't modify this function!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let content = [(Fruit::Apple, 4), (Fruit::Mango, 2), (Fruit::Lychee, 5)];
        HashMap::from_iter(content)
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let fruit_kinds = [
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);

        for fruit_kind in fruit_kinds {
            let Some(amount) = basket.get(&fruit_kind) else {
                panic!("Fruit kind {:?} was not found in basket", fruit_kind);
            };
            assert!(*amount > 0);
        }
    }
}
