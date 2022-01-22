fn main() {

    // Unlike statically sized arrays, Rust vectors allow you to store
    // lists of values that can dynamically add/remove items as needed.

    let mut v: Vec<i32> = Vec::new();

    for idx in (1..11).rev(){
        v.push(idx);
    }

    for idx in v {
        println!("Counting down: {}", idx);
    }

    println!("Liftoff!");
    

    let mut string_vec: Vec<String> = Vec::new();

    string_vec.push(String::from("Hello"));
    string_vec.push(String::from(", "));
    string_vec.push(String::from("world!"));

    for item in string_vec {
        println!("{}", item);
    }

    dropped();

    borrowing_and_ownership();

    // we can also iterate over a vector as below:
    let v_two = vec![1, 100, 1000];

    for item in &v_two {
        println!("{}", item);
    }

    // We can mutate the elements similarly by iterating as below:
    let mut v_three = vec![1, 100, 1000];

    for item in &mut v_three {
        *item += 1;
        println!("{}", item);
    }


    vec_of_different_types();
    
}


fn dropped() {

    // We can also declare a vector using this syntax.
    let v = vec![1, 2, 3, 4, 5];

    // We can reference an element of a vector via indexing
    // syntax (here we're creating a reference to the second element)
    // ...
    let third: &i32 = &v[2];
    println!("The third element by index syntax is {}", third);
    
    // Or we can use ".get()" syntax, which, when combined with a
    // match statement can help us extract the element.
    match v.get(2) {
        Some(third) => println!("The third element by .get syntax is {}", third),
        None => println!("There is no third element."),
    }

    // Note that if we have gone out of scope, the vector has "dropped"
    // all of its elements.
}


fn borrowing_and_ownership() {
    let mut v = vec![1, 2, 3, 4, 5];

    // As with the rest of Rust, we can't have mutable and immutable
    // references to the same information within the same scope.
    // This will fail, as the vector v is mutable, but the "first"
    // borrow is not, as the data at the fist index of the vector
    // could change later!
    //
    // let first = &v[0];

    // If we simply access the value at the index rather than
    // referencing the memory location of the first element
    // of the fector, this works fine.
    let first = v[0];
    println!("Got: {}", first);

    v.push(6);

    println!("The first element is: {}", first);
}


fn vec_of_different_types() {
    // Normally vectors must be of the same type. However, when
    // combined with an Enum, we can store a vector of different types!

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row: Vec<SpreadsheetCell> = Vec::new();

    row.push(SpreadsheetCell::Int(6));
    row.push(SpreadsheetCell::Float(66.77));
    row.push(SpreadsheetCell::Text(String::from("Test!")));


    let mut item: i32 = 0;
    let mut float_item: f64 = 0.0;
    let mut string_item: String = String::new();

    for val in row {

        if let SpreadsheetCell::Int(option) = val {
            item = option;
        }
        else if let SpreadsheetCell::Float(option) = val {
            float_item = option;
        }
        else if let SpreadsheetCell::Text(option) = val {
            string_item = option;
        }

    }

    println!("Got: {} - {} - {}", item, float_item, string_item);

}