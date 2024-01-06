fn main() {
    let nums = vec![3, 2, 2, 3];
    let val = 3;
    let test = Solution::remove_element(&mut nums.clone(), val);
    println!("test: {:?}", test);
    
    let nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    let test = Solution::remove_element(&mut nums.clone(), val);
    println!("test: {:?}", test);
}

struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }
        i as i32
    }
}
