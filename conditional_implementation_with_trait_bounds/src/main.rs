use std::fmt;
use std::error::Error;


struct Pair<T> {
    x: T,
    y: T
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self{
        Self { x, y }
    }
}

impl <T: fmt::Display> fmt::Display for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// We can use trait bounds to implement behaviors for a 
// generic type that are only available if the generic
// if of a type that implements the traits sepecified.
// For example, adding a special display method that
// is only available to Pair<T> types where the T type
// implements both the Display and PartialOrd traits.

impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


// Note that implementing a trait for a type also allows us
// to conditionally implement any traits that are part of
// said trait. These care called "blanket implementations".
// For example, any type that implements the Display trait
// can also implement the ToString trait.


fn main() -> Result<(), Box<dyn Error>> {
    let pair = Pair::new(5, 5);

    pair.cmp_display();

    println!("Point is: {}", pair.to_string());
    Ok(())
}
