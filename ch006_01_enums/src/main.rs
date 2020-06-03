//enums are data types that provide variants


enum IpAddrKind {
    V4,
    V6,
}

//enums in rust are more powerful. They can have associated values
enum IpAddr {
    V4(String),
    V6(String),
}

//the variants don't need to all have the same associated value types:
enum Message {
    Quit,                           //includes no other value
    Move { x: i32, y: i32 },        //includes an anonymous struct
    Write(String),                  //includes a string
    ChangeColor(i32, i32, i32),     //includes 3 i32 values
}

//enums can have an impl block as well:
impl Message {
    fn call(&self) {
        //method body
    }
}



fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    //the standard library has some useful enums, such as Option (since rust doesn't have nulls):
    //enum Option<T> {
    //  Some(T),
    //  None,
    //}
    //
}
