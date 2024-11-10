/**
Into Iterator

Definition : Converts a collection into an iterator that owns its elements, allowing for direct iteration and ownership transfer
Purpose : `IntoIterator` allows the collection itself to be consumed, useful in cases where we don't need the original collection afterward.
Difference: While `.iter()` borrows elements, `.into_iter()` takes ownership
*/

pub fn run() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    let num_iter = numbers.into_iter();

    // Consumes `numbers` directly; `numbers` can't be used after this
    for number in num_iter.into_iter() {
        println!("IntoIterator gave: {}", number);
    }
}
