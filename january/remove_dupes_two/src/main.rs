fn main() {
    let mut nums1 = vec![1, 1, 1, 2, 2, 3]; // expected return 5
    let mut nums2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3]; // expected return 7

    println!("nums1 result: {}", Solution::remove_duplicates(&mut nums1));
    println!("nums2 result: {}", Solution::remove_duplicates(&mut nums2));
}

/**
 * Given a sorted non-decreasing array of integers, remove 'some'
 * duplicates in-place such that each unique element appears at
 * most twice. The relative order should remain unchanged.
 * 
 * Since it is impossible to change the length of the array in
 * some languages, you must instead have the result be placed in
 * the first part of the nums array. More formally, if there are 'k'
 * elements after removing the udplicates, then the first 'k' elements
 * of 'nums' should hold the final result. It does not matter what you
 * leave beyond the first 'k' elements.
 * 
 * Return 'k' after placing the final result in the first 'k' slots of nums.
 * 
 * Do not allocate extra space for another array. You must do this by
 * modifying the input array in-place with O(1) extra memory.
 */
struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        
        // escape zero length array
        if nums.is_empty() {
            return 0;
        }

        let mut occurences = 1;
        let mut index = 1;

        for i in 1..nums.len() {
            // If the current value is equal to the previous value
            if nums[i] == nums[i - 1] {
                // Increment the occurences
                occurences += 1;
            } else {
                // Otherwise, reset the occurences
                occurences = 1;
            }

            // If the occurences is less than or equal to 2
            if occurences <= 2 {
                // Set the value at the index to the current value
                nums[index] = nums[i];
                // Increment the index
                index += 1;
            }
        }

        // Return the pointer as the length of the array
        index as i32
    }
}