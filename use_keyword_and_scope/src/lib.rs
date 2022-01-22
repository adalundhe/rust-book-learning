mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}

        fn seat_at_table() {}

    }

}

mod back_of_house {
    
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }

    mod serving {

        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}

    }

    mod kitchen {
        fn fix_incorrect_order() {
            cook_order();
            super::serving::serve_order();
        }
    
        fn cook_order() {}
    }
}


// The `use` keyword in Rust is similar to creating
// a symlink in a filesystem. It allows us to bring
// specific modules in a package into scope without 
//having to repeatedly use long absolute paths.

// Note that we *could* specify the path all the way
// to `add_to_wait_list()`, but this is not the idomatic
// way to use `use`.
// use crate::front_of_house::hosting;

// We can specify either using `crate` for absolute paths 
// or `self` for relative paths as below:
//
// use self::front_of_house::hosting

// We can also use the crate name instead of `crate`.

use crate::front_of_house::hosting;

// We can also "re-export" imported items. When we
// import a module, that import is private and only
// available within the scope of the module in which
// it is imported. However, if we preface `use` with
// `pub`, other modules can use the import:
//
// pub use crate::front_of_house::hosting;
//


use std::fmt::Result;
// We can alias imports (as in Python or JS)
// to avoid namespace conflicts.
use std::io::Result as IoResult;

fn function1() -> Result {}
fn function2() -> IoResult<()> {}


// In order to use external packages:
//
// - First add the package to the Cargo.toml - i.e.:
//      
//      rand = "0.8.3"
//
// Then import:
//
// use rand::Rng;
//

// We can further cleanup import syntax utilizing "use lists".
// For example, we can rewrite:
//
// use std::io;
// use std::io::Write;
//
// to:
//
// use std::{}
//    cmp::Ordering,
//    io
// }
//


// You can also import all public items defined in a path using
// the glob operator
//
// use std::collections::*;
//

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}