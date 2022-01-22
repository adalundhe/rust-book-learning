// Note that this file is lib.rs. We use
// lib.rs instead of main.rs when we want to
// compile a *library* to use in other Rust
// programs instead of a *binary. The `main.rs`
// file is the default cargo entrypoint for
// compiled binaries, and `lib.rs` for
// libraries.

mod front_of_house {
    

    pub mod hosting {

        fn add_to_wait_list() {}

        fn seat_at_table() {}

    }

    // Note that this module is private without the `pub`
    // access modifier. Any calls to it from outside the module
    // will fail.
    mod serving {

        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}

    }

}


pub fn eat_at_restaurant() {

    crate::front_of_house::hosting::add_to_wait_list();

    front_of_house::hosting::add_to_wait_list();

}