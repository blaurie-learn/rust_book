//in rust, ,everything is private by default. 

mod front_of_house {
    
    // these need to be marked pub because eat_at_restaurant cant see them otherwise. 
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//To show rust where to find an item in a module tree, we can used similar paths as if navigating a
//filesystem:
//  -Absolute path starts from the crate root
//  -Relative path starts from the current module and uses self, super, or an identifier in the
//  current module

//this function is part of the public API, so is marked with "pub"
pub fn eat_at_restaurant() {
    
    //absolute path:
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path:
    front_of_house::hosting::add_to_waitlist();
}

// the "super" keyword can be used to refer to a parent module when using a relative path

//for structs, the struct can be designated public, but the fields will remain private:
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //however, for enums, when the enum is public, all the variants become public as well
    pub enum Appetizer {
        Soup,
        Salad
    }
}
