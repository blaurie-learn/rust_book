//Mock objects are specific types of test doubles that record what happens during a test so you can
//assert that the correct action took place.


pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgen warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}


//we need a mock object that, instead of sending an email or test when we call send, will onyl keep
//track of the messages it's told to send.

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        //sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                //sent_messages: vec![]
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {

        //the signature for "send" doesn't take a mutable self.
        fn send(&self, message: &str) {

            //however, our mock needs to mutate by adding to the vector. We can't do this because
            //&self is not mutable!
            //This is a situation where interior mutability can help!
            //sent.sent_messages.push(String::from(message));
            

            //for the implementation of send(), we can still use an immutable &self and then
            //borrow_mut() the sent_messages!
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        //assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}


