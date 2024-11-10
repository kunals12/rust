/*
Basic Iterator
Definition : A simple iterator created from a collection [like a Vec] and manually advance using next().
Purpose : The basic iterator allows step by step access to elements, suitable for fined grain controls.
Difference: Unlike other types, it doesn't allow transform or filter items, just iterates through them in sequence.
 */

pub fn run() {
    let numbers: Vec<i32> = vec![1,2,3,4,5];

    let mut num_iter = numbers.iter();

    // Manually fetch each item with `next`
    while let Some(num) = num_iter.next() {
        println!("got : {}", num);
    }
}