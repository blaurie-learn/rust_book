
fn main() {
    
    //matching against literals directly:
    {
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything else"),
        }
    }

    //matching named variables
    {
        //variables declared in a match will shadow the parent scopes variables.
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }
    }

    //multiple patters:
    {
        //you can match mutiple patterns using the | syntax
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("everything else"),
        }
    }

    //matching ranges with ..
    {
        //Ranges only work for numeric values or char
        let x = 5;

        match x {
            1..=5 => println!("one through 5"),
            _ => println!("anything else"),
        }

        let x = 'c';

        match x {
            'a'..='j' => println!("early ascii letter"),
            'k'..='z' => println!("late ascii letter"),
            _ => println!("not a letter"),
        }
    }

    //destructuring to break apart values:
    {
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {}(", x, y),
        }
    }

    
    //destructuring enums:
    {
        let msg = Message::ChangeColour(0, 160, 255);

        match msg {
            Message::Quit => println!("Quit"),
            Message::ChangeColour(r, g, b) => {
                println!("Change the colour to {} {} {}", r, g, b);
            },
            _ => println!("another message"),
        }
    }

    //use underscore to ignore pattern "_", parts of a pattern (Some(_)) or variables _name, or
    //even function paramters (perhaps you no longer need it)
    
    //use .. to ignore remaining parts:
    {
        let p = Point3d { x: 0, y: 1, z: 3 };

        match p {
            Point3d { x, .. } => println!("x is {}", x),
        }
    }


    //match guards:
    {
        //match guards are an additional "if" condition specified after the mattern in a match that
        //much also match along with the mattern matching
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }


    //@ bindings
    {
        //The @ operator lets us create a binding that holds a value at the same time that we're
        //testing the value:
        let msg = Msg::Hello { id: 5 };

        match msg {
            Msg::Hello { id: id_var @ 3..=7 } => println!("Found id in range {}", id_var),
            Msg::Hello { id: 10..=12 } => println!("Found in another range"),
            Msg::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

enum Msg {
    Hello { id: i32 },
}
