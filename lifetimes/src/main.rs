// Lifetime parameters are what tells the Rust compiler
// how "long" a give reference "lives". For example,
// The below function won't compile because Rust can't
// tell if x or y is being returned and thus whether
// x or y is a dangling pointer. Introducing a lifetime 
// parameter (`'a`) helps tell Rust that regardless of 
// whether x or y is returned the borrow of x or y 
// remains valid.
//
// fn longest(x: &str, y: &str) -> &str {
//     if (x.len() > y.len()){
//         x
//     }
//     else {
//         y
//     }
// }

// The inclusion of the liftime parameter 'a here
// tells Rust that regardless of whether the ref to 
// x or y is returned, that reference will still
// be valid and can continue to be used. 
//
// The best way to think of lifetime parameters is
// as a constraint that enforces that references
// consumed and returned are valid for as long
// as the lifetime parameter is in scope (in this case
// as long as it is within the scope of the function).
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

// We can also use references with struct parameters.
// However,for struct reference params to be valid, they
// must contain a lifetime parameter.

struct Excerpt<'a> {
    part: &'a str,
}


// Note that we do *not* have to specify lifetime parameters
// for the following situations:
//
// - A function has a single parameter.
// - A method references &self or &mut self (the lifetime of self is automatically applied)
// 
// An example for the first case:
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// An example for the second case:
impl <'a>Excerpt<'a> {
    fn len(&self) -> usize{
        self.part.len()
    }
}

// Finally, it is generally enforced and recommend that each reference
// parameter of a function have its *own* lifetime parameter.

fn main() {
    let string_one = "test this";
    let string_two = "test_two";

    let result = longest(&string_one, &string_two);
    println!("The longest string is: {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // This stores a reference to the first sentence.
    let excerpt = Excerpt {
        part: first_sentence,
    };

    println!("Excerpt is: {}", excerpt.part);

    let word = first_word(&string_one);
    println!("The first word is: {}", word);

    let excerpt_length = excerpt.len();
    println!("Excerpt length: {}", excerpt_length);
}
