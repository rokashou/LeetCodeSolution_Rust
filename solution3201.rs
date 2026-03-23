impl Solution{
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut dp = [[0;2];2];
        let mut ans = 0;

        for &num in &nums {
            let x = (num % 2) as usize;
            for y in 0..2 {
                dp[x][y] = dp[y][x] + 1;
                ans = ans.max(dp[x][y]);
            }
        }
        
        ans
    }
}