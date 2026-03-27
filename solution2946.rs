// 2946. Check if Matrix Is Similar
/* 2026-03-27
Runtime: 0 ms, faster than 100.00%.
Memory Useage: 2.26 MB, less than 50.00%.
 */

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let m = mat.len();
        let n = mat[0].len();
        let k = (k as usize) % n;

        for i in 0..m {
            for j in 0..n {
                let target = if i % 2 == 0 {
                    // even row -> right shift
                    (j+k) % n
                } else {
                    // odd row -> left shift
                    (j+n-k) % n
                };
                if mat[i][j] != mat[i][target] {
                    return false;
                }
            }
        }

        true
    }
}

