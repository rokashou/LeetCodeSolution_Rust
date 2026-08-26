// 2904. Shortest Beautiful Substring
/* 2026-08-26 
Runtime: 0 ms, faster than 100.00%.
Memory Usage: 2.14 MB, less than 77.78%.

*/


impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;

        let mut left: usize = 0;
        let mut ones: usize = 0;
        let mut ans_start: usize = 0;
        let mut ans_len: usize = 0;
        let mut has_ans = false;

        for right in 0..n {
            if bytes[right] == b'1' {
                ones += 1;
            }
            
            // Shrink the left limit:
            // 1) ones > k -> must to shrink
            // 2) ones == k, but there is 0 at the left -> can shrink to shorten the length

            while ones > k || (ones == k && bytes[left] == b'0') {
                if bytes[left] == b'1' {
                    ones -= 1;
                }
                left += 1;
            }

            if ones == k {
                let len = right - left + 1;
                if !has_ans || len < ans_len {
                    has_ans = true;
                    ans_start = left;
                    ans_len = len;
                } else if len == ans_len {
                    // compare dictionary order when length is the same
                    let cand = &bytes[left..left + len];
                    let cur = &bytes[ans_start..ans_start + ans_len];
                    if cand < cur {
                        ans_start = left;
                        ans_len = len;
                    }
                }
            }
        }

        if has_ans {
            // finally create the answer string from the bytes slice
            String::from_utf8(bytes[ans_start..ans_start + ans_len].to_vec()).unwrap()
        } else {
            String::new()
        }
    }
}
