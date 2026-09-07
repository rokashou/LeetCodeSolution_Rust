/* 940. Distinct Subsequences II */
/* Runtime: 0ms, Memory: 2.23MB */


impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        // total: number of distinct subsequences so far, including the empty one.
        let mut total: i64 = 1;

        // last[c]: value of 'total' right before character c was last appended.
        let mut last: [i64; 26] = [0; 26];

        for ch in s.bytes() {
            let idx = (ch - b'a') as usize;
            let new_total = ((2 * total - last[idx]) % MOD + MOD) % MOD;
            last[idx] = total; // record total BEFORE this occurrence is counted
            total = new_total;
        }

        // subtract 1 to remove the empty subsequence, adjust for possible negative mod
        ((total - 1 + MOD) % MOD) as i32
    }
}

