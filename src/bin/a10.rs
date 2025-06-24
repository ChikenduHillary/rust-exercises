// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn check_value(is_greater_than_100: bool) {
    match is_greater_than_100 {
        true => println!("greater than 100"),
        false => println!("less than or equal to 100")
    }
}

fn main() {
    let number = 100;

    let is_greater_than_100 = if number > 100 {
        true
    } else if number <= 100 {
        false
    } else {
        false
    };

    check_value(is_greater_than_100);
}
