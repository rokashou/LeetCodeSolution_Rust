use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n3 = nums.len();
        let n = n3 / 3;
        let mut left_min = vec![0i64; n3];
        let mut max_heap = BinaryHeap::new();
        let mut left_sum: i64 = 0;

        // 處理前2n個元素中的最小n個數
        for i in 0..2 * n {
            left_sum += nums[i] as i64;
            max_heap.push(nums[i]);
            if max_heap.len() > n {
                if let Some(val) = max_heap.pop() {
                    left_sum -= val as i64;
                }
            }
            if max_heap.len() == n {
                left_min[i] = left_sum;
            }
        }

        let mut min_heap = BinaryHeap::new(); // 用 Reverse 模擬 min-heap
        let mut right_sum: i64 = 0;
        let mut ans = i64::MAX;

        // 處理後2n個元素中的最大n個數（即從末尾往前）
        for i in (n..n3).rev() {
            right_sum += nums[i] as i64;
            min_heap.push(Reverse(nums[i]));
            if min_heap.len() > n {
                if let Some(Reverse(val)) = min_heap.pop() {
                    right_sum -= val as i64;
                }
            }
            if min_heap.len() == n {
                ans = ans.min(left_min[i - 1] - right_sum);
            }
        }

        ans
    }
}
