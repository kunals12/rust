/*
Iterator Adaptors

Definition : Iterator adaptors are methods that transform or filter items from an iterator chain (e.g., `map`, `filter`).
Purpose : Adaptors allow you to modify or filter elements without changin the original collection, creating a new iterator.
Difference : Unlike a basic iterator, adaptors are "lazy" and produce values only when needed [e.g., when 'collect' is called]
*/

pub fn run() {
    let numbers: Vec<i32> = vec![1,2,3,4,5];

    // `map` adaptor: transforms each item (here, squares each number)
    let squares: Vec<_> = numbers.iter().map(|&x| x*x).collect();
    println!("Squares: {:?}", squares);

    // `filter` adaptor: selects only items that meet a condition (hers, even numbers)
    let evens: Vec<_> = numbers.iter().filter(|&&x| x%2 ==0).collect();
    println!("Evens : {:?}", evens);

    // `take` adaptors: limits the number of items returned (here, takes the first 2 items)
    let takes: Vec<_> = numbers.iter().take(2).collect();
    println!("Takes: {:?}", takes);
}