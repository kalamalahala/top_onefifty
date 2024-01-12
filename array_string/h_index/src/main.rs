fn main() {
    let h1 = Solution::h_index(vec![3,0,6,1,5]);
    let h2 = Solution::h_index(vec![1,3,1]);
    println!("h1 = {}", h1);
    println!("h2 = {}", h2);
}

struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut list = citations.clone();
        list.sort_unstable();
        let length = list.len();
        let mut h = 0;
        for index in 0..length {
            let cit_count = list[index];
            let valid_count = length - index;
            h = h.max(valid_count.min(cit_count as usize));
        }
        h as i32
    }
}