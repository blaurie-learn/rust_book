//the book goes through examples:
//uses a width + height variable to calculate the area.
//Then it groups these as a tuple because that makes a bit more sense.
//Then it proceeds to the following:


//There are certain derivable traits. Traits that provide some sort of default implementation that
//can be inferred for you.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 11,
    };

    println!("The area is: {}", area(&rect));
    println!("For the rectangle: {:?}", rect);      //:? calls debug() instead of display()
}
