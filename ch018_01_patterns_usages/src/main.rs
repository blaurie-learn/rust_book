//a pattern consists of some conbination of the following:
//  literals
//  destructured arrays, enums, structs, or tuples
//  variables
//  wildcards
//  placeholders


//Patterns do work the same every place they can be used. In some places, patterns must be
//irrefutable, in others they can be refutable.

fn main() {
    //all the places patterns can be used:
    //match ARMS
    {
        //match VALUE {
        //  PATTERN => EXPRESSION,
        //  PATTERN => EXPRESSION,
        //}
        let val = Some(56);
        match val {
            Some(v) => println!("Got some({})", v),
            None => println!("Oh No!"),
        }

        //match must be exhaustive -- all variants must be able to get into a branch
        //  _ will match anything, so can be useful as the last branch
        //
    }



    //if let expressions
    {
        //if let can have a corresponding else, else if, or else if let
        //Note that if let doesn't force exhaustive checking

        let favourite_colour: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(colour) = favourite_colour {
            println!("Using your favourite colour, {}, as the background", colour);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30  {
                println!("Using purple as the background colour."); 
            } else {
                println!("Using orange as the background colour.");
            }
        } else {
            println!("Using b lue as the background colour");
        }
    }


    //while let conditional loops
    {
        //similar to if let, but runs as long as a pattern continues to match
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }



    //for loops:
    {
        //in a for loop, the value immediately following the "for" is the pattern
        //so, for x in y, the x is the pattern
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }


    //let statements
    {
        //let PATTERN = EXPRESSION;
        //  "bind what matches the pattern to PATTERN

        let (x, y, z) = (1, 2, 3);
    }

    //function parameters:
    fn foo(&(x, y): &(i32, i32)) {
        println!("The current location is {}, {}", x, y);
    }

}
