struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// We don't have to used named fields in Rust, and
// can instead provide a tuple. You can't reference
// fields by name, so this functions more as a "named tuple".
// This lets you create named types with tuples that otherwise
// function as tuples.
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

// Note that we're using String and not &str here. This is because
// we want the data for a struct to remain valid for the lifetime
// of a struct, so surrenduring ownership makes sense. We *can*
// use "lifetimes" to ensure an instance of a struct has ownership 
// of data assigned from another variable while allow the variable
// to retain ownsership of the original data.
fn build_user(username: String, email: String) -> User{
    // Note that like JS w/ objects, we can just pass in matching variable
    // names to a struct.

    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

// This is a "unit-like struct". It has no fields and is akin to `()`, the
// "unit type" (default returned by Rust functions)
// struct AlwaysEqual;


fn main() {
    let mut user_one = User {
        active: true,
        username: String::from("Test"),
        email: String::from("test@test.com"),
        sign_in_count: 1,
    };
    
    user_one.email = String::from("test2@test.com");

    println!("User Active - {}", user_one.active);
    println!("User Name - {}", user_one.username);
    println!("User Email - {}", user_one.email);
    println!("User Sign-In Count - {}", user_one.sign_in_count);

    let user_two = build_user(String::from("Test Two"), String::from("test3@test.com"));

    println!("User Active - {}", user_two.active);
    println!("User Name - {}", user_two.username);
    println!("User Email - {}", user_two.email);
    println!("User Sign-In Count - {}", user_two.sign_in_count);

    // Like JS with objects we can use spread-lie "update" syntax to build
    // new instances of a struct from a current struct's values.

    let user_three = User {
        email: String::from("test4@test.com"),
        ..user_one
    };

    println!("User Active - {}", user_three.active);
    println!("User Name - {}", user_three.username);
    println!("User Email - {}", user_three.email);
    println!("User Sign-In Count - {}", user_three.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("I have the color black as - R: {}, G: {}, B: {}", black.0, black.1, black.2);

    println!("The origin is at - X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);

    // let subject = AlwaysEqual;


}
