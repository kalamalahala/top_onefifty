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
        println!("nums: {:?}", nums);
        let mut jumps = 0;
        println!("jumps made: {}", jumps);
        let mut current_jump_end = 0;
        println!("current_jump_end: {}", current_jump_end);
        let mut farthest = 0;
        println!("farthest possible: {}", farthest);
        for (i, _) in nums.iter().enumerate().take(nums.len() - 1) {
            println!("=====================");
            println!("index: {}", i);
            farthest = std::cmp::max(farthest, i + nums[i] as usize);
            println!("farthest possible: {}", farthest);
            println!("is current index the end of the current jump?");
            if i == current_jump_end {
                println!("yes");
                jumps += 1;
                println!("jumps made: {}", jumps);
                current_jump_end = farthest;
                println!("current_jump_end: {}", current_jump_end);
            }
            println!("finished iterations");
            println!("=====================");
        }
        jumps
    }
}
