use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::error::Error;

// When panic!() is called, there is NO way for the code to recover. However,
// returning a Result type lets us determine how the calling code can/should
// handle any errors. Result is thus the best choice for handling functions
// or behaviors that might fail.

// However, in tests, when prototyping code, or in example code, panicing
// can be a better option (for example, we want or tests execution to end
// when a test fails). The only other general exception is if
// you have more information than the Rust compiler and *know* that a
// given bit of code can't fail, for example.
//
//      use std::net::IpAddr;
//
//      let home: IpAddr = "127.0.0.1".parse().unwrap();


// Rather than just implicitly return the unit tuple in main `()`,
// we can instead return a Result of the unit tuple or a safe
// pointer (Box type) to a generic error type (which allows us
// to return any error early). Box<dyn Error> is called a 
// "trait object".
fn main() -> Result<(), Box<dyn Error>> {
    // The Result enum returns either a Type T or
    // Error E. Methods that return the Result
    // type (like File::open() below) can take 
    // advantage of this by using match syntax.
    // let f = File::open("hello.txt");

    // let data = match f {
    //     Ok(file) => file,
    //     // Here we use the panic! macro but use our own error message.
    //     Err(error) => panic!("Problem opening file: {:?}", error),
    // };

    // We can extend this to match different types of errors.

    let file_two = File::open("hello.txt");

    let mut file_data = match file_two {
        Ok(file) => file,

        // Here we utilize an additional match to map the error kind
        // to either creating the file if it does not exist (or panicing
        // if the creation fails) or panicing if the file exists but
        // another type of error (such as a read error) occured.
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error)
            }
        }

        // This can be useful for building robust programs that handle
        // common error cases (such as a file not existing) rather
        // than just erroring.
    };

    let new_data = String::from("Hello!");
    file_data.write_all(new_data.as_bytes()).expect("Error writing string!");

    // However, this is verbose. Instead we can use the .unwrap() and
    // .expect() methods. The .unwrap() method is like out first
    // match example, which will return the Ok(<args>) option of
    // a Result enum or call the panic! macro.

    let mut file_three = File::open("hello.txt").unwrap();
    let mut data = String::new();
    file_three.read_to_string(&mut data).expect("Error reading!");
    println!("{}", data);

    // The .expect() method expands upon this by letting us
    // provide a custom message to the panic!() macro:

    let file_four = File::open("hello.txt").expect("Failed to open hello.txt");

    let result = read_username_from_file().expect("Failed to load data!");
    
    let result_two = read_username_from_file_two().expect("Error reading file!");
    println!("Got: {}", result_two);
    
    let last_char = another_chained_example(&result_two.to_string()).expect("Error parsing last char!");
    println!("Got last char: {}", last_char);

    Ok(())

}


fn read_username_from_file() -> Result<String, io::Error> {
    // We can also propagate errors by returning them to the calling code.
    // For example:
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // This lets us continue to pass down the error, deferring
    // the panic until the function returns.
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_two() -> Result<String, io::Error> {
    // A less verbose way to propagate errors is to use the ?
    // operator, which functions exactly as the match statements
    // above.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // Note that the ? operator can ONLY be used on methods that
    // return the Result or Option types.

}

fn read_username_from_file_three() -> Result<String, io::Error> {
    // Like in JS, we can use the ? operator to conditionally chain
    // method calls.
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

}


fn another_chained_example(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}