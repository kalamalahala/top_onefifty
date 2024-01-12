/**
 * You are climbing a staircase. It takes 'n' steps to reach the top. Each time
 * you can either climb 1 or 2 steps. In how many distinct ways can you climb to
 * the top?
 */
fn main() {
    let n1 = 2; // 2
    let n2 = 3; // 3
    let n3 = 4; // 5
    let n4 = 5; // 8
    let n5 = 45; // almost two billion

    println!("climbing two stairs: {}", Solution::climb_stairs(n1));
    println!("climbing three stairs: {}", Solution::climb_stairs(n2));
    println!("climbing four stairs: {}", Solution::climb_stairs(n3));
    println!("climbing five stairs: {}", Solution::climb_stairs(n4));
    println!("climbing 45 stairs: {}", Solution::climb_stairs(n5));
}

struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        let mut stairnacci = vec![0; n as usize + 1];
        // stairnacci[0] = 0; // not needed
        stairnacci[1] = 1; // first stair
        stairnacci[2] = 2; // second stair

        for stair in 3..=n as usize {
            // we can get to the current stair by either taking one step from the previous stair
            // or two steps from the stair before that
            // so the number of ways to get to the current stair is the sum of the number of ways
            // to get to the previous stair and the number of ways to get to the stair before that
            stairnacci[stair] = stairnacci[stair - 1] + stairnacci[stair - 2];
        }

        // return the number of ways to get to the nth stair
        stairnacci[n as usize]
    }
}
