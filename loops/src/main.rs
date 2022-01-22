fn main() {

    let mut counter = 0;

    let mut result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("Beginning liftoff!");

    while result > 0 {
        println!("Lifting off in - {}", result);
        result -= 1;
    }

    println!("LIFTOFF!");

    for number in (1..11).rev(){
        println!("Counting down- {}", number);
    } 
}
