fn main() {
    let nums = vec![2,2,1,1,1,2,2];
    let result = Solution::majority_element(nums);
    println!("result = {}", result);

    let nums = vec![3,2,3];
    let result = Solution::majority_element(nums);
    println!("result = {}", result);
}

struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut counts: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for num in &nums {
            // iterate over the vector and count the number of times each element appears
            let count = counts.entry(*num).or_insert(0);
            // increment the count inside the hashmap
            *count += 1;
        }

        counts.into_iter().max_by_key(|&(_, count)| count).unwrap().0
    }
}