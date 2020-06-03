//methods are like function but defined within the context of a struct


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//in rust, the impl block defines methods (among other things)
impl Rectangle {

    //note the &self. This takes a reference. We could also take &mut self for a mutable reference.
    //We don't need the : Rectangle, it is inferred for the self parameter
    fn area(&self) -> u32 {

        //Also notice that the period operator automatically dereferenced. no need for ->
        self.width * self.height
    }

    //a method with other parameters:
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //another useful feature is associated function. We've already seen one (String::from).
    //often used for constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

//note that each struct can have multiple impl blocks
impl Rectangle {
    fn perimeter(&self) -> u32 {
        (self.width * 2) + (self.height * 2)
    }
}


fn main() {
    let rect = Rectangle {
        width: 10,
        height: 11,
    };

    let rect2 = Rectangle {
        width: 11,
        ..rect
    };

    let _rect3 = Rectangle::square(30);

    println!("The area for {:?} is : {}", rect, rect.area());
    println!("Rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("The perimiter of rect is: {}", rect.perimeter());
}
