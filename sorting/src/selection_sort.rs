pub fn run() {
    let mut arr: Vec<i32> = vec![4, 2, 7, 5, 8];

    // Outer loop:
    // i represents the current position we are trying to fix (sort)
    for i in 0..arr.len() - 1 {

        // Assume the current index has the minimum value
        let mut min = i;

        // Inner loop:
        // Search for the smallest element in the remaining unsorted part
        for j in i + 1..arr.len() {

            // If we find a smaller value, update 'min'
            if arr[j] < arr[min] {
                min = j;
            }
        }

        // After inner loop, 'min' holds the index of the smallest element
        // in the unsorted portion

        // ---- Swap arr[i] and arr[min] ----
        let temp = arr[min];   // store smallest value temporarily
        arr[min] = arr[i];     // move current value to min position
        arr[i] = temp;         // place smallest value at correct position
        // -------------------------------
    }

    println!("Sorted array: {:?}", arr);
}
