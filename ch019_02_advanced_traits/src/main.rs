//Associated types connect a type of placeholder with a trait such that the trait method
//definitions can use placeholder types in their signature.

//One example of associated types is Iterator:
//pub trait Iterator {
//  type Item;
//  fn next(&mut self) -> Option<Self::Item>;
//}
//
//Implementors of this trait will specify the concrete type for Item and the next method will
//return an Option containing that concrete type.
//
//So why not just use a generic? When a trait has a generic, it can be implemented multiple times
//for a type, changing the generic type parameter each time.
//Associated types done need to annotate types because you cant implement this trait on a type
//multiple times. We can only choose what type Item will be once. 




//_________________
//When we use Generic type parameters, we can specify a default concrete type. This eliminates the
//need for implementors of the trait to specify a concrete type if the default type works.
//  The syntax is <PlaceholderType=ConcreteType> when declaring the generic type
//
//A good example where htis technique is useful is with operator overloading. Rust doesn't let you
//create operators or overload arbitary operators, but you can overload operations of corresponding
//traits for exisitng operators.
//
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

//The add trait has a generic type:
//trait Add<Rhs=Self> {
//  type Output;
//  fn add(self, rhs: Rhs) -> Self::Output;
//}
//
//The "Rhs=Self" part is called a "default type parameter". If a type for RHS is not specified,
//then the type will default to "Self" (which will be the type that Add is being implemented on.







//Qualitying the trait method for disambiguation:
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("this is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up Up and away");
    }
}

impl Human {
    fn fly(&self) {
        println!("flaping arms furiously");
    }
}

//which implementation shoud be called?
fn fly_human(human: &Human) {
    human.fly();        //flapping arms furiously

    //Here's how we call the other implementation to disambiguate them:
    Pilot::fly(human);
    Wizard::fly(human);
}




use std::fmt;
//Specify that a trait requires another trait to be implemented (Supertrait)
trait Outline: fmt::Display {   //Outline requires that the type you impl on also impls Display
}










fn main() {
    let human = Human{ };
    fly_human(&human);
}
