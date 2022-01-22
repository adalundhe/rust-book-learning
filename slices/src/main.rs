// fn first_word(s: &String) -> usize {
//     // This returns the index of a space separating words, thus the end index
//     // of the first word.

//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' '{
//             return i;
//         }
//     }

//     s.len()

// }

fn first_word_by_slice(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]

}


fn main() {
    let s = "hello world!";

    // The index 5 is returned, but if we mutate the variable 
    // "s" (as with the .clear() below) then we can't meanigfully
    // use the index returned by `first_word`.
    let word = first_word_by_slice(&s);

    println!("The first word is - {}", word)
}
