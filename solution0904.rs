use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, usize> = HashMap::new();
        let mut left = 0;
        let mut max_len = 0;

        for right in 0..fruits.len() {
            *count.entry(fruits[right]).or_insert(0) += 1;

            while count.len() > 2 {
                let fruit = fruits[left];
                if let Some(c) = count.get_mut(&fruit) {
                    *c -= 1;
                    if *c == 0 {
                        count.remove(&fruit);
                    }
                }
                left += 1;
            }

            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}

