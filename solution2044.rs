use std::{collections::HashMap, mem::swap};

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut level: HashMap<i32, usize> = HashMap::new();
        let mut next: HashMap<i32, usize>; 
        for num in nums {
            next = level.clone();
            for (prev_val, prev_freq) in &level {
                let next_val = *prev_val | num;
                *next.entry(next_val).or_insert(0) += prev_freq;
            }
            *next.entry(num).or_insert(0) += 1;
            swap(&mut level, &mut next);   
        }
        // for (k, v) in &level {
        //     println!("==> {} - {}", k, v);
        // }
        let max_or = *level.keys().max().unwrap();
        level[&max_or] as i32
    }
}