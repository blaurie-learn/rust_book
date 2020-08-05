use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //another way of making a hashmap is to use iterators and the collect method:
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    //for types implemeting the copy trait, like i32, values are copied into the hashmap.
    //for owned values like String, the values will be moved and the hashmap will be the owner
    //of those values

    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //we are no longer able to use field_name and field_value here.
    
    //if we insert references into the hashmap, the value wont be moved into the hashmap. The
    //values that the references point to must be valid for at least as long as the hash map is
    //valid.

    //accessing values:
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    //keep in mind that each key is unique and can have only one value. inserting to a key that
    //exists wil loverwrite the previous value.

    //only inserting for a key that doesn't have a value:
    scores.insert(String::from("Blue"), 20);
    
    //entry will return a Result
    scores.entry(String::from("")).or_insert(50);

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    //example counting words in a sentence:
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
       *count += 1;
    }

    println!("{:?}", map);

    //by default HashMap uses a cryptographically strong hashing function that can provide
    //resistance to Denial of Service attacks
    //you can switch to a different function by specifying a hasher. A hasher is a type that
    //implements the BuildHasher trait.
}
