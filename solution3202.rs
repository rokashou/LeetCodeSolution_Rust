impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![0;k];k];
        let mut ans = 0;

        for &num in &nums {
            let x = (num % k as i32) as usize;

            for i in 0..k{
                dp[i][x] = dp[x][i] + 1;
                ans = ans.max(dp[i][x]);
            }
        }

        ans
    }
}