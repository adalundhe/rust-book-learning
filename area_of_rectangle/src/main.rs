

// By default we can't print out our struct because it doesn't implement 
// the Debug "Trait" (a collection of methods). By adding the derived Debug
// trait, we tell the println! macro how to handle printing the struct.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// By using impl, we can add methods to a struct. Note
// we need to have an argument of &self as to reference the specific
// instance of a rectangle and not have that instance loose
// ownership of itself. 

// Note that all functions declared within a single impl block are associated
// with the given type. Associated methods do *not* need to reference &self
// to be associated - which can be useful for adding behaviors that do not require
// referencing the instance's own data. HOWEVER, these methods cannot be
// accessed via .method() syntax and called from an instance. They must be implemented 
// as -
//
//  let square = Rectangle::square();
//  
// called from the *type* and called using "::" syntax. You can use multiple
// "impl" blocks for any type.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn is_bigger(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    fn chained_copy(self, other: &mut Rectangle) -> Self {
        other.height = self.height;
        other.width = self.width;

        self
    }
}

fn main() {

    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    // Note the {:?}. The ":?" indicates debug formatting.
    println!("Rectangle is - {:?}", rectangle);

    // We can also excplicitly use the debug macro - dbg!().
    let rectangle_two = Rectangle {
        width: dbg!(30),
        height: dbg!(30),
    };

    // We're derefing here to get the values.
    dbg!(&rectangle_two);

    let rectangle_area = rectangle.area();

    println!("Area for rectangle one is - {}", rectangle_area);

    // Note that calling a method on a struct (i.e. - struct.method()) is the same as - (&struct).method().
    println!("Rectangle One is bigger than Rectanble Two - {}", rectangle.is_bigger(&rectangle_two));

    let mut rectangle_three = Rectangle::square(16);
    println!("Area for rectangle one is - {}", rectangle_three.area());

    let mut rectangle_four = Rectangle::square(16);

    rectangle
        .chained_copy(&mut rectangle_three)
        .chained_copy(&mut rectangle_four);

    
    println!("Area for rectangle one is - {}", rectangle_three.area());
    println!("Area for rectangle one is - {}", rectangle_four.area());
    
}
