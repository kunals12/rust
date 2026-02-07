/*
You are given a sorted array of integers and a target value.
Your task is to determine whether the target exists in the array using binary search logic.

If the target is found, return the index where it is located.
If the target is not found, return -1.

📥 Input
An integer array arr of length n
An integer target
arr = [2, 4, 6, 8, 10, 12, 14]
target = 10


📤 Output
An integer representing the index of target in arr
Return -1 if target does not exist
Output is 4
 */
fn main() {
    let arr = vec![1,2,3,4,5,6,7,8,9,10];
    let r: usize = arr.len() - 1;
    let l:usize = 0;
    let target: i8 = 7;
    println!("{r}, {l}, {target}");
    let ans = search(arr, l, r, target);
    // println!("{}", ans);
}

fn search(arr: Vec<i8>, l: usize, r:usize, target:i8)  {
    if l > r {
        println!("Number {} can't be found in array", target);
    }

    let mid = (l + r)/2;
    println!("mid is {}", mid);

    if arr[mid] == target {
        println!("Num is found at index: {}", mid);
    } else if arr[mid] > target {
        search(arr, l, mid - 1, target);
    } else {
        search(arr, mid + 1, r, target);
    }
}
