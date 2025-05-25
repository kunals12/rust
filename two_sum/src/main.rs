use std::collections::HashMap;

fn main() {
    let arr1: Vec<i32> = vec![3, 2, 4];
    println!("Indices: {:?}", two_sum(arr1, 6)); // Should print: Indices: [0, 1]
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, value) in nums.iter().enumerate() {
        let a = target - value;
        println!("{}", a);

        if let Some(index) = map.get(&a) {
            println!("{}", index);
            return vec![*index, i as i32];
        }

        map.insert(*value, i as i32);
    }

    vec![]
}
