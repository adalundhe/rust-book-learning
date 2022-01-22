// If we ue a semicolon after a module declaration instead
// of a block (i.e. `{}`) this tells Rust we want to load the
// contents of the module from another file. In this case,
// we want to load the `hosting` module from the like-named
// `hosting.rs` file, contained in a `front_of_house` folder
// that shares the parent module's name.

pub mod hosting;