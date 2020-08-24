//closures are anonymous functions you can save in a variable or pass as arguments to other
//functions.

//closures are great for code reuse and allowing behavior customization


use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    //this only does the math once, but there's a spot that doesn't use this math at all
    //and it still blocks the rest
    //let expensive_result = simulated_expensive_calculation(intensity);

    //So we can define a closure instead:
    //let expensive_closure = |num| {
    //    println!("calculating slowly...");
    //    thread::sleep(Duration::from_secs(2));
    //    num
    //};

    //If we wanted to write that closure with types:
    //let expensive_closure = |num: u32| -> u32 { ... };
    //
    //For closures, quite a lot of syntax is optional in a lot of cases:
    //  fn  add_one_v1   (x: u32) -> u32 { x + 1 }      //function
    //  let add_one_v2 = |x: u32| -> u32 { x + 1 };     //closure
    //  let add_one_v3 = |x|             { x + 1 };     //closure
    //  let add_one_v4 = |x|               x + 1  ;     //closure
    
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });


    if intensity < 25 {
        //println!("Today, do {} pushups!", simulated_expensive_calculation(intensity));
        //println!("Next do {} situps!", simulated_expensive_calculation(intensity));
        //This is expensive to do more than once. We'll extract it up

        //println!("Today, do {} pushups!", expensive_result);
        //println!("Next do {} situps!", expensive_result);
        
        //println!("Today, do {} pushups!", expensive_closure(intensity));
        //println!("Next do {} situps!", expensive_closure(intensity));

        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next do {} situps!", expensive_result.value(intensity));



    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

//Then we have a better solution which lets us lazy evaluate a cache a result
//A struct needs a type for a variable, so closures have a type. 
//  All closures implement at least one of the traits:
//      Fn, FnMut, FnOnce. Talk about that later. This example can use Fn
struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> 
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    //closures can also capture environment.
    let x = 4;
    let equal_to_x = |y| x == y;

    let z = 4;
    assert!(equal_to_x(z));

    //Even though x was defined in this scoe, it was still able to be used in the closure. You
    //can't do that with functions.
    //fn equal_to_x(y: i32) -> bool {
    //    x == y
    //}                     //compielr error!! x not in scope

    //When closures capture a value from the body, there is overhead in the form of memory to store
    //those copies variables. Functions don't incur that overhead.
    
    //Closures can capture their environment in 3 ways:
    //  FnOnce  -Consumes the variables it captures from its enclosing scope, known as the closures
    //          environment. To consume, the closure takes ownership of variables and moves them
    //          into the closure when it is defined. Once part of the name represents the fact that
    //          the closure cant take ownership of the same variables more than once, so it can
    //          only be called once.
    //
    //  FnMut   -can change the environment because it mutable borrows values
    //
    //  Fn      -borrows values immutably
    //
    //  For the most part, rust can infer which trait to use based on the closures values and it
    //  environemnt.
    //
    //  If you want to force a closure to take ownership of the values it uses, you can use the
    //  "move" keyword before the parameter list (mostly useful when passing parameter data to a
    //  new thread).
    
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    //println!("Can't use x here: {:?}", x);
    
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}


