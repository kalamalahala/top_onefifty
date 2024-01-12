/**
 * You are given an integer array 'nums'. You are initially
 * positioned at index 0, and each element represents your
 * maximum 'jump' length at that position. Return 'true' if
 * you can reach the last index, or 'false' otherwise.
 */
fn main() {
    let j1 = vec![2, 3, 1, 1, 4]; // true
    let j2 = vec![3, 2, 1, 0, 4]; // false
    let j3 = vec![1, 1, 1, 1, 1]; // true
    let j4 = vec![1, 2, 3, 4, 5]; // true
    let j5 = vec![5, 4, 3, 2, 1]; // true

    println!("can jump: {}", Solution::can_jump(j1));
    println!("can jump: {}", Solution::can_jump(j2));
    println!("can jump: {}", Solution::can_jump(j3));
    println!("can jump: {}", Solution::can_jump(j4));
    println!("can jump: {}", Solution::can_jump(j5));
}

struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut possible_distance = 0;
        for (index, _) in nums.iter().enumerate() {
            if index > possible_distance {
                return false;
            } else {
                possible_distance = std::cmp::max(possible_distance, nums[index] as usize + index);
            }
        }

        true
    }
}