// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut store = HashMap::new();
    let mut total = 0;

    store.insert(5, "chairs".to_owned());
    store.insert(3, "beds".to_owned());
    store.insert(2, "tables".to_owned());
    store.insert(0, "couches".to_owned());

    for (amount, item) in store.iter() {
        total = total + amount;
        if *amount == 0 {
            println!("item:{:?} amount:{:?}", item, amount);
        } else {
            println!("item:{:?} amount: out of stuck", item);

        }
    }

    println!("total amout of item: {:?}", total)


}
