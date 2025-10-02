// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl  Person {
    fn new(name: &str, age: u8) -> Result<Self, String> {
        if age >= 21 {
            Ok(Self { name: name.to_owned(), age })
        } else {
            Err("You must be at least 21 to make a restricted purchase".to_owned())
        }
    }
}

fn main() {
  let adult = Person::new("Hillary", 25);
  let child = Person::new("victory", 15);

  match adult {
      Ok(e) => println!("{:?}", e),
      Err(e) => println!("{:?}", e)
  }

  match child {
      Ok(e) => println!("{:?}", e),
      Err(e) => println!("{:?}", e)
  }
}
