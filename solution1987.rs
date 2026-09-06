/* 1987. Number of Unique Good Subsequences */
/* Runtime: 1ms, Memory: 2.52MB */

impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut end0: i64 = 0;
        let mut end1: i64 = 0;
        let mut has_zero = false;

        for c in binary.bytes() {
            if c == b'1' {
                end1 = (end0 + end1 + 1) % MOD;
            } else {
                end0 = (end0 + end1) % MOD;
                has_zero = true;
            }
        }

        let mut result = (end0 + end1) % MOD;
        if has_zero {
            result = (result + 1) % MOD;
        }
        result as i32
    }
}
