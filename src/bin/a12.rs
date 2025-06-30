// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Dimensions {
    width: f64,
    height: f64,
    breath: f64
}

impl Dimensions {
    fn print(&self) {
        println!("width={:?}", self.width);
        println!("height={:?}", self.height);
        println!("breath={:?}", self.breath);
    }
}

enum Color {
    Red,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Blue => println!("Blue")
        }
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    color: Color,
    weight: f64,
}

impl ShippingBox {
    fn new(dimensions: Dimensions, color: Color, weight: f64) -> Self {
        Self {dimensions, color, weight}
    }

    fn print(&self) {
        println!("weight={:?}", self.weight);
        self.color.print();
        self.dimensions.print();
    }
}

fn main() {
    let small_box = Dimensions {
        height: 33.3,
        width: 23.4,
        breath: 35.6
    };

    let new_box = ShippingBox::new(small_box, Color::Red, 50.0);
    new_box.print();
}
