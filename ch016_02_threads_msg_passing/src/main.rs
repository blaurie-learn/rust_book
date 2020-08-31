//message passing is an increasingly popular way to ensure safe concurrency.
//For this, one major tool rust has is the channel.
//  A channel has a transmitter and a receiver.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    {
        let (tx, rx) = mpsc::channel(); //Multiple Producer, Single Consumer

        //move the transmitter into a spawned thread and have it send one string
        thread::spawn(move || { 
            let val = String::from("hi");
            tx.send(val).unwrap();

            //note that send takes ownership of val and it is no longer usable in case the other thread
            //tries to modify it!
            //println!("val is {}", val);
        });

        //recv will block till a value is received or an error is the transmitter closes the channel.
        //try_recv will not block, but will return a Result<T, E> immediately.
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    {
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });


        for received in rx {
            println!("Got: {}", received);
        }
    }
}
