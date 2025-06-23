// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn get_cordinates() -> (i32, i32) {
    (2390, 5)
}

fn main() {
    let (_x, y) = get_cordinates();

    if y > 5 {
        println!("y cordinate is greater than 5");
    } else if y < 5 {
        println!("y cordinate is less than 5");
    } else {
        println!("y cordinate is equal 5");
    }
}