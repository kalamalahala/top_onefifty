fn main() {
    let mut nums = vec![1,2,3,4,5,6,7];
    let k = 3;
    Solution::rotate(&mut nums, k);
    println!("nums = {:?}", nums);

    let mut nums = vec![-1,-100,3,99];
    let k = 2;
    Solution::rotate(&mut nums, k);
    println!("nums = {:?}", nums);
}

struct Solution {}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // if nums.len() < 2 {
        //     return;
        // }
        // let mut k = k;
        // while k > 0 {
        //     let replacement = nums.swap_remove(0);
        //     nums.insert(1, replacement);
        //     k -= 1;
        // }

        // rust has a function for this, rotate_right
        let k = k as usize % nums.len(); // k could be larger than nums.len()
        nums.rotate_right(k);
    }
}