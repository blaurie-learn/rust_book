fn main() {
    //In rust, iterators are lazy, meaning they have no behavior until there is some method called
    //that consumes them.

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got {}", val);
    }

    //all iterators implement the Iterator Trait defines in the standard library which looks like:
    //pub trait Iterator {
    //  type Item;
    //  fn next(&mut self) -> Option<Self::Item>;
    //  //other methods not shown
    //}
    //
    //Notice the "type Item" and "Self::Item" which are defining an associated type with this
    //trait. In order to implement an iterator, you also need to define an "Item" type, and this
    //"Item" type is used as the return of the next method.
    //
    //using an iterator directly:
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    //Note
    //  iter() produces immutable references
    //  into_iter() takes ownership of v1 and returns owned values
    //  iter_mut() iterates over mutable references

    //Methods that call next are consuming adaptors, such as sum:
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("Total is: {}", total);

    //Plus there are methods that produce other iterators (Iterator adaptors), like map:
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

//We also have filter, which is neat:
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

//We can implement our own iterators:
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0 }
    }
}

impl Interator for Counter {
    type Item = u32;   // Associated type is i32

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
