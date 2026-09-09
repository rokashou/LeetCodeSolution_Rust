/* 127. Word Ladder */
/* 14ms, 2.88MB */

use std::collections::HashSet;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut dict: HashSet<String> = word_list.into_iter().collect();
        if !dict.contains(&end_word) {
            return 0;
        }
        dict.remove(&begin_word);

        let mut begin_set: HashSet<String> = HashSet::new();
        begin_set.insert(begin_word);
        let mut end_set: HashSet<String> = HashSet::new();
        end_set.insert(end_word);

        let mut level = 1;

        while !begin_set.is_empty() && !end_set.is_empty() {
            // Always expand the smaller frontier to minimize branching
            let (smaller, larger) = if begin_set.len() <= end_set.len() {
                (begin_set, end_set)
            } else {
                (end_set, begin_set)
            };

            let mut next_set: HashSet<String> = HashSet::new();

            for word in &smaller {
                let mut chars: Vec<char> = word.chars().collect();
                for i in 0..chars.len() {
                    let original = chars[i];
                    for c in 'a'..='z' {
                        if c == original {
                            continue;
                        }
                        chars[i] = c;
                        let candidate: String = chars.iter().collect();
                        // Meeting the other frontier means a path is found
                        if larger.contains(&candidate) {
                            return level + 1;
                        }
                        if dict.remove(&candidate) {
                            // mark visited by removing from dict
                            next_set.insert(candidate);
                        }
                    }
                    chars[i] = original; // restore before trying next position
                }
            }

            begin_set = next_set;
            end_set = larger;
            level += 1;
        }

        0
    }
}
