fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if *item > *largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest_number = largest(&number_list);
    println!("The largest number is {}", largest_number);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest_char = largest(&char_list);
    println!("The largest char is {}", largest_char);
}
