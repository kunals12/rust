mod basic_iterators;
mod iterator_adaptors;
fn main() {
    println!("Basic Iterator Example:");
    basic_iterators::run();

    println!("Iterator Adaptors Example");
    iterator_adaptors::run();
}
