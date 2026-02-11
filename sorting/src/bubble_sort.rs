pub fn run() {
    let mut numbers: Vec<i32> = vec![4, 5, 8, 9, 1, 6];

    // Outer loop controls number of passes
    for i in 0..numbers.len() {

        // swap = 0 means: "no swap happened in this pass yet"
        let mut swap: i32 = 0;

        // Inner loop compares adjacent elements
        // We reduce the range by 'i' because last i elements are already sorted
        for j in 1..numbers.len() - i {

            // If left element is greater than right element, they are in wrong order
            if numbers[j - 1] > numbers[j] {

                println!("Swapping {} and {}", numbers[j - 1], numbers[j]);

                // ----- SWAP LOGIC -----
                let temp = numbers[j - 1];   // Store left value temporarily
                numbers[j - 1] = numbers[j]; // Move right value to left
                numbers[j] = temp;           // Put stored left value to right
                // -----------------------

                // Mark that a swap happened in this pass
                swap = 1;
            }
        }

        // If no swap happened in this pass, array is already sorted
        if swap == 0 {
            break;
        }
    }

    println!("Bubble Sorted array: {:?}", numbers);
}
