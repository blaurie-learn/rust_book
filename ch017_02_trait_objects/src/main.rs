//Ch017 01 was just talking about OO peroperties of rust.

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {

    //Box<dyn Draw> is a "Trait Object" -- A stand in for any type inside a Box that implements
    //"Draw".
    //This is obviously different from a generic type parameter. A generic can only be substituted
    //with one type at a time. This Box<dyn "trait"> object allows - at runtime - any concrete
    //object that implements Draw.
    pub components: Vec<Box<dyn Draw>>,
}


//imagine:
//  pub struct Screen<T: Draw> {
//      pub components: Vec<T>,
//  }
//
//The "components" can only hold one concrete type, whereas a "screen" typically hold many
//components that would implement draw, such as button, layout, dropdown, etc...


impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        //code to actually draw a button
        println!("Draw a button {}", self.width);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        //code to- acually draw a button
        println!("Draw a select box");
    }
}



//Object safety is required for for trait objects. A trait if object safe if all the methods
//defined in the trait have the following properties:
//  -The return type isn't "Self" (capital S)
//  -There are no generic type parameters


fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
