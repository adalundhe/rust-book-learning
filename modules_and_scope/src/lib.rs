// Note that this file is lib.rs. We use
// lib.rs instead of main.rs when we want to
// compile a *library* to use in other Rust
// programs instead of a *binary. The `main.rs`
// file is the default cargo entrypoint for
// compiled binaries, and `lib.rs` for
// libraries.

mod front_of_house {
    
    // Without the `pub` specifier, we can't access any of the functions/code
    // inside the `hosting` module, as Rust makes all module code private by
    // default.
    pub mod hosting {

        // Note that even if the module is specified as public, the methods within
        // are still private unless specified as public via the `pub` access modifier.
        pub fn add_to_wait_list() {}

        fn seat_at_table() {}

    }

}

mod back_of_house {

    // We can specify a publicly accessible struct like this.
    pub struct Breakfast {
        // And then further specify publicly available attributes or struct methods
        // as below.
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

    // We can create a publicly accessible enum like this. Note
    // that, unlike a struct, all Enum variants are public if
    // the Enum is public.
    pub enum Appetizer {
        Soup,
        Salad
    }

    // Note that this module is private without the `pub`
    // access modifier. Any calls to it from outside the module
    // will fail.
    mod serving {

        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}

    }

    mod kitchen {
        fn fix_incorrect_order() {

            // Calling other code specified within the same module can be done
            // as expected.
            cook_order();
    
            // We can start paths at the parent module (front_of_house in this case)
            // by using `super`. This is akin to using `..<module>` (going up a directory)
            // to import a module via a relative path in Python or JS.
            super::serving::serve_order();
        }
    
        fn cook_order() {}
    }
}


pub fn eat_at_restaurant() {

    // This is and absolute path import, akin to calling from the base of 
    // a package in Python. For example:
    //
    //  zebra_harness.server import Server
    //
    crate::front_of_house::hosting::add_to_wait_list();

    // This is a relative path import, akin to:
    //
    // .server import Server
    //
    // in Python
    front_of_house::hosting::add_to_wait_list();

    // We want to use absolute path imports more often than not,
    // as they insure we can better move definitions and calls to
    // those definitions.

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);

}