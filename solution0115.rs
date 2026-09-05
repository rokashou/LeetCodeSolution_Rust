/* 115. Distinct Subsequences */
/* 
    Runtime: 3ms, beats 50.00% 
    Memory: 2.08MB, beats 100.00%
 */

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let n = s.len();
        let m = t.len();

        if m > n {
            return 0;
        }

        // use u64 to avoid overflow
        let mut dp: Vec<u64> = vec![0u64; m + 1];
        dp[0] = 1; // empty string is a subsequence of any string

        for i in 1..=n {
            // iterate backwards to ensure that we are using the previous row's values
            for j in (1..=m).rev() {
                if s[i - 1] == t[j - 1] {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[m] as i32
    }
}

