// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    favorite_color: String,
}


fn print_details(name: &str, color: &str) {
   println!("name: {:?}, favorite color: {:?}", name, color)
} 
fn main() {

        let store = vec![
        Person {
            name: "victor".to_owned(),
            age: 9,
            favorite_color: "blue".to_owned(),
        },
        Person {
            name: "victor".to_owned(),
            age: 10,
            favorite_color: "blue".to_owned(),
        }
    ];

    for item in store {
        if item.age >= 10 {
            print_details(&item.name, &item.favorite_color);
        }
    }

}
