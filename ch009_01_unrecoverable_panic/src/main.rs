//when the panic! macro executes, your program will print a failure message, unwind and clean up
//the stack, and then quit by default.
////you can change this behavior to immediately abort and let the operating system clean up your
//memory by adding "panic='abort'" t the appropriate profile sections in your Cargo.toml
//
//[profile.release]
//panic = 'abort'
//




fn main() {
    //panic!("crash and burn");
    
    let v = vec![1, 2, 3];

    v[99];
    //note how we are told that we can set RUST_BACKTRACE=1 environemtn variable to get a backtrace
    //of exactly what happened to cause the error. In order to get large, informative backtraces,
    //debug symbols must be enabled (which they are by default when not using the --release flag.
}
