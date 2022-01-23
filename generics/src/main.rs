use std::ops;
use std::convert;
// We can declare templated functions as below.
// Note that for this example we need to:
//
// - Implement this Sized trait for `list`.
//
// - Implement the PartialOrd trait for comparison with T item/current_largest
//
// fn largest<T>(list: &[T]) -> T {
//     let mut current_largest = list[0];
//
//     for &item in list {
//         if item > current_largest {
//             current_largest = item;
//         }
//     }
//
//     current_largest
//
// }

// Generic structs are also a possibility!
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}


// This is how we declare a Trait, a collection of behaviors that types can implement! Note
// that we also use `where T: ...` here to implement additional Traits (from above) for type
// T so that we can before math operations on the generic type! If we define implementations
// for the trait methods here, they will be treated as default implementations.
trait Calculate<T> {
    fn add(self, other: Point<T>) -> Point<T> where T: ops::Add<Output=T> + Copy + Clone;
    fn sub(self, other: Point<T>) -> Point<T> where T: ops::Sub<Output=T> + Copy + Clone;
    fn mul(self, other: Point<T>) -> Point<T> where T: ops::Mul<Output=T> + Copy + Clone;
    fn sqrd(self) -> Point<T> where T: ops::Mul<Output=T> + Copy + Clone;
    // These are Trait Boundaries using the `where` syntax as opposed to:
    //
    //      fn distance<T: convert::Into<64> + ops::Add<Output=T> + ...>(self, other: Point<T>) -> f64 {}
    //
    // The `where` syntax is preferred due to readability.
    //
    fn distance(self, other: Point<T>) -> f64 where T: convert::Into<f64> + ops::Add<Output=T> + ops::Sub<Output=T> + ops::Mul<Output=T> + Copy + Clone;
}


// We can implement Traits as below for our generic. Note that the signatures of the actual
// implementations MUST exactly match their trait declaration above.
impl <T>Calculate<T> for Point<T> {
    fn add(self, other: Point<T>) -> Point<T> 
        where T: ops::Add<Output=T> + Copy + Clone {
            Point {
                x: self.x + other.x,
                y: self.y + other.y
            }
        }

    fn sub(self, other: Point<T>) -> Point<T>
        where T: ops::Sub<Output=T> + Copy + Clone {
            Point {
                x: self.x - other.x,
                y: self.y - other.y
            }
        }

    fn mul(self, other: Point<T>) -> Point<T>
        where T: ops::Mul<Output=T> + Copy + Clone {
            Point {
                x: self.x * other.x,
                y: self.y * other.y
            }
        }

    fn sqrd(self) -> Point<T>
        where T: ops::Mul<Output=T> + Copy + Clone{
            Point {
                x: self.x * self.x,
                y: self.y * self.y
            }
        }

    fn distance(self, other: Point<T>) -> f64 
        where T: convert::Into<f64> + ops::Add<Output=T> + ops::Sub<Output=T> + ops::Mul<Output=T> + Copy + Clone{
        let dist_sqrd = other.sub(self).sqrd();
        (dist_sqrd.y + dist_sqrd.x).into().sqrt()
    }
    
}


fn main() {

    let point = Point { x: -5, y: 5 };
    println!("Point is: {} - {}", point.x, point.y);
    let other_point = Point { x: 8, y: 8};

    let dist = point.distance(other_point);
    println!("Got: {}", dist);

}
