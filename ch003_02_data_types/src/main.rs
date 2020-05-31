//rust is statically typed
//the compiler can often infer types, but a type must be known at compile time

//scalar types:
//  8-bit   i8  u8
//  16-bit  i16 u16
//  32-bit  i32 u32 (default)
//  64-bit  i64 u64
//  128-bit i128    u128
//  arch    isize   usize
//
//  int literals:
//      Decimal     98_222
//      Hex         0xff
//      Octal       0o77
//      Binary      0b111_000
//      Byte (u8 only)  b'A'
//
//  Integer overflow:
//      In debug mode, integer overflow checks are added. An overflow will cause a panic
//      In release mode, rust performs twos compliment wrapping

//float types:
//  32-bit  f32
//  64-bit  f64 (default)


fn main() {
    //addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;

    //remainder
    let remainder = 43 % 5;

    //rust has booleans which can be true of false
    let t = true;
    let f: bool = false;        //explicit type annotation

    //rust has 'char' which is the most primitive alphabetic type
    //rusts char type is a 4-byte Unicode Scalar Value. It is more than just ASCII.
    //valid values are U+0000 - U+D7FF and U-E000 - U+10FFFF inclusive
    //a character isn't really a concept in unicode. This will be discussed further later.
    let c = 'z';


    //rust has compound types as well
    //Compound Types can group multiple values in to one type
    //Rust has two primitive compound types: tuples and arrays
    
    //tuple:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //destructuring a tuple:
    let (x, y, z) = tup;
    let five_hundred = x.1;

    //arrays:
    let a = [1, 2, 3, 4, 5];

    let b: [i32, 5] = [1, 2, 3, 4, 5];

    //the same as writing [3, 3, 3, 3, 3]:
    let threes = [3; 5];

    //rust will panic if you attempt to access a value out of bounds:
    let index = 10;
    let el = threes[index];         //panic!
    

    
}
