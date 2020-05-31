//it'd be pretty annoying to have to take and give ownership all the time, so rust provides
//references and borrowing:


//at any given time you can have either one mutable reference or any number of immutable references
//references must always be valid


fn main() {
    let mut s = String::from("Hello, world");

    let sz = calculate_length(&s);          //& mean take a reference
                                            //lets you take a reference without transferring
                                            //ownership

    change(&mut s);     //a wild logic bug appears

    //keep in mind that we can take as many immutable borrows as we like, but once a mutable borrow
    //occurs, we will get in trouble:
    let r1 = &s;        //no problem
    let r2 = &s;        //no problem
    let r3 = &mut s;    //big problem, will fail to compile once the variable is used:
    //println!("values are {} {} {}", r1, r2, r3);    //compile fails

    println!("size of {} is {}", s, sz);
}

//since the parameter s is &String (a reference to string), it is a borrow and does not get dropped
//at the end of the function
fn calculate_length(s: &String) -> usize {
    s.len()
}

//we can also take a mutable reference
fn change(some_str: &mut String) {
    some_str.push_str("!");
}

//rust also prevents dangling references (this particular example is, somewhat ironically, created
//because of RAII).
//this function causes compile to fail because it is detecting a borrow from a value that is
//dropped without a lifetime specified (lifetimes will come later
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s  //return a reference to s
//}   //s has "drop" called and is freed
