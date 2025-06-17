// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Apple,
    Orange,
    Grape
}

struct Drinks {
    name: String,
    flavor: Flavors,
    price: i32
}

fn print_drink(drink: Drinks) {

    match drink.flavor {
        Flavors::Apple => println!("Flavor Apple"),
        Flavors::Orange => println!("Flavor Orange"),
        Flavors::Grape => println!("Flavor Grape")
    }

    println!("Name {}", drink.name);
    println!("Price {}", drink.price);
}

fn main() {

    let apple_drink = Drinks {
        name: "Apple Juice".to_string(),
        flavor: Flavors::Apple,
        price: 10
    };

    print_drink(apple_drink);

    let orange_drink = Drinks {
        name: "Orange Juice".to_string(),
        flavor: Flavors::Orange,
        price: 20
    };

    print_drink(orange_drink);


}
