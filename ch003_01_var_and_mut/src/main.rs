//by default variable are immutable. add mut for mutability

fn main() {
    let x = 5;
    println!("the value of x is: {}", x);

    //this wont compile without making x mutable
    //x = 6;
    println!("the value of x is: {}", x);

    //rust also has constants which cannot be made mutable
    const MAX_POINTS: u32 = 100_000;

    //as previously see, rust lets you shadow variables:
    let x = x + 2;
    println!("The value of x is: {}", x);
}
