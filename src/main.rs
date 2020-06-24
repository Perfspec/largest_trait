fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if *item > *largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a, 'b, 'c>(x: &'a str, y: &'b str, result: &'c mut String) {
    if x.len() > y.len() {
        *result = (*x).to_string();
    } else {
        *result = (*y).to_string();
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest_number = largest(&number_list);
    println!("The largest number is {}", largest_number);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest_char = largest(&char_list);
    println!("The largest char is {}", largest_char);
	
	let string1 = String::from("long string is long");
	let mut result = String::new();
    {
        let string2 = String::from("xyz");
        longest(string1.as_str(), string2.as_str(), &mut result);
    }
    println!("The longest string is {}", result);
}
