/**
 * You are given an array 'nums' of length n.
 * Each element nums[i] represents the maximum
 * length of a jump from index i. You can assume
 * that you can always reach the last index, but
 * I will write exception cases as well.
 * 
 * Return the minimum number of jumps required.
 */
fn main() {
    // let j1 = vec![2, 3, 1, 1, 4]; // 2
    // let j2 = vec![2, 3, 0, 1, 4]; // 2
    let j3 = vec![1, 2, 1, 1, 1]; // 3

    // println!("jumps required: {}", Solution::jump(j1));
    // println!("jumps required: {}", Solution::jump(j2));
    println!("jumps required: {}", Solution::jump(j3));
}

struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut current_jump_end = 0;
        let mut farthest = 0;
        for (i, _) in nums.iter().enumerate().take(nums.len() - 1) {
            farthest = std::cmp::max(farthest, i + nums[i] as usize);
            if i == current_jump_end {
                jumps += 1;
                current_jump_end = farthest;
            }
        }
        jumps
    }
}
