use rand::Rng;
// Note that matches are "exhaustive", meaning that if
// you don't cover all cases, Rust will not compile the
// statement. This prevents "fall through" (like not having
// a default case on a switch statement).
fn plus_one(x: Option<i32>) -> i32 {
    match x {
        None => 1,
        Some(i) => i + 1,
    }
}

struct Actions {
}

impl Actions {

    fn add_fancy_hat() {
        println!("Added fancy hat!");
    }

    fn remove_fancy_hat() {
        println!("Removed fancy hat!");
    }

    fn move_player(spaces: u8) {
        println!("Player moved - {} - spaces!", spaces)
    }

    fn reroll() {
        println!("Re-rolling!");
    }
}

fn dice_roll() -> u8 {
    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen_range(1..10);

    number
}


// Note here we can use the placeholder `_` as the "default" case
// if there's no match.
fn take_action(roll: u8) {

    match roll {
        1 => Actions::move_player(roll),
        3 => Actions::add_fancy_hat(),
        7 => Actions::remove_fancy_hat(),
        _ => Actions::reroll()
    }

}


// We can also use a variable to cover all cases. In 
// this case we're using `other`, such that any
// case that doesn't match (for roll) defaults to
// calling Action::move_player(spaces: u8)
fn take_action_with_move(roll: u8) {
    match roll {
        1 => Actions::add_fancy_hat(),
        3 => Actions::remove_fancy_hat(),
        other => Actions::move_player(other),
    }
}


// We can also use `_` (the placeholder) and 
// `()` (the unit/empty tuple type) to tell
// Rust that if no value matches we don't want to
// run any code!
fn take_action_with_nothing(roll: u8) {
    match roll {
        1 => Actions::move_player(roll),
        3 => Actions::add_fancy_hat(),
        7 => Actions::remove_fancy_hat(),
        _ => ()
    }
}


fn main() {
    let five = Some(5);

    let six = plus_one(five);

    let one = plus_one(None);

    println!("Got - {} and {}", six, one);

    let mut roll = dice_roll();
    take_action(roll);

    roll = dice_roll();
    take_action_with_move(roll);

    roll = dice_roll();
    take_action_with_nothing(roll);
    
}
