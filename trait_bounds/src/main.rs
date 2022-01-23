// We can use trait boundaries to make our "largest"
// function work. We need to implement the PartialOrd
// and Copy traits (the former for comparison with `>`,
// the latter to make sure we borrow correctly).
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut current_largest = list[0];

    for &item in list {
        if item > current_largest {
            current_largest = item;
        }
    }

    current_largest

}


// We can alternatively define trait bounds 
// using the "where" keyword.
fn largest_with_where<T>(list: &[T]) -> T
    where T: PartialOrd + Copy {
        let mut current_largest = list[0];

        for &item in list {
            if item > current_largest {
                current_largest = item;
            }
        }

        current_largest
    }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_with_where(&char_list);
    println!("The largest char is {}", result);
}
