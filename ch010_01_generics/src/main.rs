
//look at two functions:
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


//we can parameterize the types with generics
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {         //note here we are assuming some properties of T
                                    //in particular, that it has the trait PartialOrd
            largest = item;
        }
    }

    largest
}


//generics in a struct definition:
struct Point<T> {
    x: T,
    y: T,
}

struct TwoPoint<T, U> {
    x: T,
    y: U,
}

//Implement methods on structs and enums and use generic types in their definition.
//Note that in this case, we need to declare the <T> just after impl to state that we want to
//implement methods on Pint<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//We can implement methods on a specific generic type of Point by specifying it:
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//The parameterization of a methods generics don't necessarily need to match the impl. For example,
//there may be a case where the method returns a new type that differs from the origin class:
impl<T, U> TwoPoint <T, U> {
    fn mixup<V, W> (self, other: TwoPoint<V, W>) -> TwoPoint<T, W> {
        TwoPoint {
            x: self.x,
            y: other.y,
        }
    }
}

//with enum definitions (As we've already seen
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 5.0 };
    //let wont_work = Point { x: 1, y: 5.0 };
    let will_work = TwoPoint { x: 1, y: 5.0 };
}
