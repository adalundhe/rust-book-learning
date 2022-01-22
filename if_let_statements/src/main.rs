fn configure_max(max_candidate: u8, config_max: &mut u8) {

    // This functions the same as:
    //
    // let config_max = Some(3u8);
    //
    // match config_max {
    //   Some(max_candidate) => {
    //     println!("The maximum is configured to be - {}.", max_candidate);
    //     *config_max = max_candidate;      
    //   },
    //   _ => println!("Max remains - {} - candidate - {} - is not equal.", *config_max, max_candidate),
    // }
    if let Some(max_candidate) = Some(*config_max) {
        println!("The maximum is configured to be - {}.", max_candidate);
        *config_max = max_candidate;
    }
    else {
        println!("Max remains - {} - candidate - {} - is not equal.", *config_max, max_candidate);
    }

}


fn main() {
    
    let mut max = 3u8;


    println!("Initial max - {}", max);
    
    configure_max(3u8, &mut max);

    println!("New max - {}", max);

}
