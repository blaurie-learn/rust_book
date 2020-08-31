//we can instead use mutexes to modify shared memory from multiple threads, but only allowing one
//thread to access at once.

//you must remember to acquire the lock before using the data
//When you're done, you must remember to unlock the data

use std::thread;
use std::sync::{Arc, Mutex};


fn main() {
    {
        let m = Mutex::new(5);

        {
            //the call to lock() returns a MutexGuard which is a smart pointer wrapped in LockResult
            //(which was unwrapped). MutexGuard implements Deref to access the data.
            //When the MutexGuard goes out of scope, it unlocks itself.
            let mut num = m.lock().unwrap();

            *num = 6;
        }

        println!("m = {:?}", m);
    }

    {
        //Arc is "Atomic Reference Counted". It is basically Rc but with counters wrapped in thread
        //safe primitives.
        let counter = Arc::new(Mutex::new(0));

        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                
                //Also notice how the Mutex gives us interior mutability
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
