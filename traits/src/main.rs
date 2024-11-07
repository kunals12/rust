struct Square {
    side: f32,
    color: String,
    line_width: u8
}

struct Rectangle {
    height: f32,
    width: f32,
    color: String,
    line_width: u8
}

trait Draw {
    fn draw_obj(&self);
}

trait Shape:Draw {
    fn area(&self) -> f32;
}

impl Square {
    fn new(side: f32, line_width: u8, color: String) -> Square {
        Square {side, color, line_width}
    }
}

impl Rectangle {
    fn new(height: f32, width: f32, color: String, line_width: u8) -> Rectangle {
        Rectangle {
            height, width, color, line_width
        }
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square: f32 = self.side * self.side;
        println!("Square area is {area_of_square}");
        area_of_square
    }
}

impl Draw for Square {
    fn draw_obj(&self) {
        println!("Sqaure color is {}", self.color);
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect: f32 = self.height * self.width;
        println!("Area of Rectangle is {}", area_of_rect);
        area_of_rect
    }
}

impl Draw for Rectangle {
    fn draw_obj(&self) {
        println!("Rectangle color is {}", self.color);
    }
}

fn main() {
    let square = Square::new(5.0, 8, String::from("Red"));
    Square::area(&square);
    Square::draw_obj(&square);

    let rectangle: Rectangle = Rectangle::new(8.0, 4.5, String::from("Blue"), 4);
    Rectangle::area(&rectangle);
    Rectangle::draw_obj(&rectangle);
}