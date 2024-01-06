fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1);

    let mut nums1 = vec![1];
    let m = 1;
    let mut nums2 = vec![];
    let n = 0;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1);

    let mut nums1 = vec![0];
    let m = 0;
    let mut nums2 = vec![1];
    let n = 1;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1);
}

/**
 * https://leetcode.com/problems/merge-sorted-array/
 * 
 * Given two integer arrays 'nums1' and 'nums2', sorted in non-decreasing order(?), and two
 * integers 'm' and 'n', representing the number of elements in 'nums1' and 'nums2' respectively.
 * 
 * Merge nums2 into nums1 as one sorted array.
 * 
 * nums1.length == m + n
 * nums2.length == n
 */
struct Solution {}
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut [i32], _n: i32) {
        let mut zipper: Vec<i32> = nums1.iter().take(m as usize).chain(nums2.iter()).cloned().collect();
        zipper.sort();
        nums1.clear();
        nums1.extend(zipper);
    }
}