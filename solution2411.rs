impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];
        let mut last_pos = vec![-1; 32]; // 每個 bit 最後出現的位置

        for i in (0..n).rev() {
            for b in 0..32 {
                if (nums[i] >> b) & 1 == 1 {
                    last_pos[b] = i as i32;
                }
            }

            let mut max_last = i as i32;
            for &pos in &last_pos {
                if pos != -1 {
                    max_last = max_last.max(pos);
                }
            }

            res[i] = (max_last - i as i32 + 1);
        }

        res
    }
}

