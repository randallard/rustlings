// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 5);
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("tomato"), 2);
    //basket.insert(String::from("ferrari"), 2);
    //basket.insert(String::from("gelding"), 2);

    // TODO: Put more fruits in your basket here.

    basket
}

fn all_fruits(fruit_map :HashMap<String,u32>) -> bool {

    let fruit_list = String::from("banana,apple,pear,tomato");
    let mut all_fruit = true;
    for (key, _value) in &fruit_map {
        if( all_fruit ) {
            all_fruit = fruit_list.contains(key); 
        }
    }
    all_fruit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3 && all_fruits(basket));
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5 && all_fruits(basket));
    }
}
