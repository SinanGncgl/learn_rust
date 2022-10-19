struct Rectangle {
    width: u16,
    height: u16,
}

impl Rectangle {
    fn area(self) -> u16 {
        self.width * self.height
    }
}

fn main() {
    let rec = Rectangle{width:4, height:5};
    println!("Area of the rectangle: {}", rec.area());
    // println!("Area of the rectangle: {}", rec.area()); in order to use rec again you have to pass as a reference: &self
}