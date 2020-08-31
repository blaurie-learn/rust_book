//Rust raw threads a 1:1 (Use the OS API for creating threads).
//there are crates for M:N threads (Green threads, or programming language provided)

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //join blocks the current thread so the joined one can finish.
    handle.join().unwrap();


    //the move keyword is often used alongside thread::spawn because it lets you use data from one
    //thread in another thread. Move forces the closure to take ownership of the values it uses in
    //the environment
    let v = vec![1, 2, 3];

    //rust doesn't know how long v will live after it moved to another thread. So the compiler
    //complains. We can move it here to have the thread take ownership.
    //let handle = thread::spawn(|| {
    let handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v);
    });

    handle.join().unwrap();
}
