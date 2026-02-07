/*Given an array of non-negative integers, write a code to search the position of an element in the array in the reverse order. Do not print anything when the element is not present in the given array.
Also, if the key is repeated, print the index where the key is appearing for the first time in reverse order.

The code should :
Take the size of the array as an input from the user.
The elements of the array as an input from the user.
The key you are searching for, as an input from the user.

Sample Input:
7
6 8 3 5 9 1 2
9
Sample Output:
2
Here the output is 2 because the position of number 9 is 2 from the reverse order as mentioned in the question.
*/

use std::num;

fn main() {
    let arr: Vec<i32> = vec![6, 8, 3, 5, 9, 1, 2];
    let target: i32 = 9;

    for (i, num) in arr.iter().enumerate().rev() {
        if num == &target {
            let var = (arr.len() - 1) - i;
            println!("{} {}", i, var)
        }
    }
}
