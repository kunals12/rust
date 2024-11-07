fn find_largest_number<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn main() {
    let x = vec![34, 56, 23, 50, 90];

    let largest = find_largest_number(&x);
    let chars = vec!['d', 'a', 'r', 'p'];
    let largest_char = find_largest_number(&chars);

    println!("The largest number is {largest}");
    println!("The largest number is {largest_char}");
}
