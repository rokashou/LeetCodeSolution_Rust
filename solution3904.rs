/* 3904. Smallest Stable Index II */
/* Runtime: 4ms, Beats 42.86% */
/* Memory: 4.18MB, Beats 57.14% */

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        // Step 1: Precompute suffix minimum: right[i] = min(nums[i..n])
        let mut right = vec![0i32; n];
        right[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            right[i] = right[i + 1].min(nums[i]);
        }

        // Step 2: Single left-to-right pass maintaining prefix maximum
        let mut left = 0i32;
        for i in 0..n {
            left = left.max(nums[i]); // running max(nums[0..=i])
            if left - right[i] <= k {
                return i as i32; // first (smallest) stable index found
            }
        }

        -1 // no stable index exists
    }
}
