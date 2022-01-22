// The option Enum lets us use null values in Rust (Rust
// doesn't have null values by default), with a templated
// type T for any fiven type and None for if it doesn't exist.
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {

    // We do not need to explicitly create or invoke Option::Some or Option::None,
    // and can just use them as below.None

    let some_number = Some(5);
    let some_string = Some("A string!");

    let absent_number: Option<i32> = None;

    println!("Got: {:?} and {:?} and {:?}", some_number, some_string, absent_number);

    // let another_number: i8 = 6;
    // 
    // This doesn't work because `some_number` is of type Option<i8> and
    // `another_number` is of type i8. We need to covert `some_number` to
    // a type T of i8 before adding, as this ensures `some_number` is not
    // null.
    //
    // let sum = some_number + another_number;
    
}
