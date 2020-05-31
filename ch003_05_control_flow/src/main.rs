fn main() {
    let number = 5;


    //brackets not required
    if number > 5 {
        println!("condition true");
    } else {
        println!("Condition false");
    }

    //there is also else if when needed.
    
    //blocks in rust can return, so we can do the following:
    let num = if number < 5 { 3 } else { 7 };

    println!("Num is: {}", num);

    //rust has 3 loops: loop, while and for

    //loop executes forever until you tell it to stop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);

    //the while loop
    let mut while_num = 3;

    while while_num != 0 {
        println!("{}!", while_num);
        while_num -= 1;
    }


    //using for to loop through a collection
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //rust has ranges as well, which are nice to run code a couple times:
    for numb in (1..4).rev() {
        println!("{}", numb);
    }
}
