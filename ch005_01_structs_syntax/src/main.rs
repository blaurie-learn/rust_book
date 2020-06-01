struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    //note how the email and username parameter names match the struct field names, so we don't
    //need to explicitely match the variables to the fields like:
    //email: email
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    //using the struct defined above:
    //notice we don't need to match the same ordering
    //also note that you cannot mark an individual field as mutable. The whole instance will be mut
    let user1 = User {
        email: String::from("user@email.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    //to get a specific value:
    println!("The email is: {}", user1.email);

    //we can also create instances from other instances using the struct update syntax:
    let user2 = User {
        email: String::from("one"),
        username: String::from("onename"),
        ..user1         //remaining fields not explicitely set should be set from user1
    };
}


//there is also a thing called a tuple struct, which is like a struct, but doesn't have names
//associated with the fields:
//each struct is its own type but can otherwise be used like a tuple
struct Color(i32, i32, i32);
struct point(i32, i32, i32);

//there are also unit () like structs. These have no fields. They are useful for when you want to
//implement traits on some type but don't have any data that you need to store in the type itself.


//It is possible for structs to store references to data owned by something else, but to do so
//requires the use of lifetimes (discussed later).

