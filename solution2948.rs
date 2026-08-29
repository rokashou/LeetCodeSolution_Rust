/*
    Runtime: 20ms
    Memory: 4.99MB
*/

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();
        
        // 依「值」排序的索引陣列
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by_key(|&i| nums[i]);
        
        let mut result = vec![0; n];
        
        let mut i = 0;
        while i < n {
            let mut j = i;
            // 相鄰(排序後)差值 <= limit 則屬於同一連通群組
            while j + 1 < n && nums[idx[j + 1]] - nums[idx[j]] <= limit {
                j += 1;
            }
            
            // 收集此群組原本的下標，由小到大排序
            let mut group_indices: Vec<usize> = idx[i..=j].to_vec();
            group_indices.sort_unstable();
            
            // 已排序的值依序填回排序後的下標
            for k in 0..group_indices.len() {
                result[group_indices[k]] = nums[idx[i + k]];
            }
            
            i = j + 1;
        }
        
        result
    }
}
