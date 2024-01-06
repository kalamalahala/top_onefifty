fn main() {
    let mut nums1 = vec![1, 1, 2]; // expected output 2
    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]; // expected output 5
    let len = Solution::remove_duplicates(&mut nums1);
    let len2 = Solution::remove_duplicates(&mut nums2);
    println!("len: {}, nums: {:?}", len, nums1);
    println!("len: {}, nums: {:?}", len2, nums2);

}

/**
 * Given an integer array sorted in non-decreasing order,
 * remove the duplicates in-place such that each unique
 * element appears only once. The relative order of the
 * elements should be kept the same. Return the number of
 * unique elements in nums.
 */
struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // if a vector is sorted, then duplicates will be next to each other
        // dedup() removes consecutive duplicates in the vector
        nums.dedup();
        nums.len() as i32
    }
}