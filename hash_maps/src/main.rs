use std::collections::HashMap;


fn main() {
    // Hashmaps are exactly as they are in C/C++.
    let mut scores = HashMap::new();

    // We insert values as below.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    println!("Got: {:?}", scores);

    // We can also construct a hashmap as below.

    let initial_scores = vec![10, 50];

    let teams = vec![
        String::from("Blue"),
        String::from("Yellow")
    ];

    // Read only hashmap.
    let scores_again: HashMap<_, _> = teams.into_iter().zip(
        initial_scores.into_iter()
    ).collect();
    
    println!("Got - {:?}", scores_again);

    // Note that for types that implement the Copy trait (like i32)
    // values are copied into the hashmap, so any variabled of those
    // types can continue to be used. However, for owned types like String,
    // values and ownership of said values will be transferred to the
    // hashmap.

    let field_name = String::from("favorite_color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // We cannot use field_name or field_value anymore as
    // the hashmap now owns the data of both variables.

    let field_int_name: i32 = 16;
    let mut field_int_value: i32 = 78;
    let mut num_map = HashMap::new();

    num_map.insert(field_int_name, field_int_value);

    // This is fne because i32 implements the Copy trait.
    field_int_value += 20;
    println!("Got: {:?} - {:?} - and {}", map, num_map, field_int_value);

    // We use the .get() method to access values from a Hashmap:
    let key = String::from("favorite_color");
    let value = map.get(&key);
    if let Some(option) = value {
        println!("Value of - {} - for key - {}", option, key);
    }


    // We can also iterate over a hashmap:
    for (key, value) in &scores {
        println!("Team {} has score {}!", key, value);
    }


    // We can update/overwrite values in a hashmap using the .insert()
    // method and just providing the same key.
    scores.insert(String::from("Blue"), 50);
    if let Some(option) = scores.get(&String::from("Blue")) {
        println!("Team Blue now has score of - {}!", option);
    } 

    // We can likewise only update an entry if it has no value by
    // using both the .entry() and .or_insert() methods:
    scores.entry(String::from("Orange")).or_insert(35);
    scores.entry(String::from("Yellow")).or_insert(20);
    for (key, value) in &scores {
        println!("Team {} now has score {}!", key, value);
    }

    // Note that .or_insert() returns a mutable reference
    // to a hashmap value if the corresponding entry exists.
    // If not, it inserts the new value for the new key
    // and then returns a mutable reference to the new
    // k/v pair.

    // The .or_insert() method can also allow us to
    // update a value based on the old value as below:
    let mut word_counts = HashMap::new();

    let text = "Hello wonderful wonderful world";
    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word counts: {:?}", word_counts);

    // Note that, by default, the hash function used by Hashmap (SipHash)
    // is not designed for performance but rather resistance to DoS attacks
    // involving hash tables. If we want to use a faster hashing algorithm,
    // we need only specify a different *hasher*, which is a type that
    // implements the BuildHasher trait.
}
