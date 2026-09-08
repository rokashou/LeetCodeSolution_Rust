/* 126. Word Ladder II */
/* 7ms, 2.44MB */

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();

        // endWord must exist in the dictionary, otherwise no path is possible
        if !word_set.contains(&end_word) {
            return vec![];
        }

        // beginWord should not be be reused as an intermediate word
        word_set.remove(&begin_word);

        // parents[word] = all predecessor words that can transform into "word"
        // on some shortest path
        let mut parents: HashMap<String, Vec<String>> = HashMap::new();

        let mut current_level: HashSet<String> = HashSet::new();
        current_level.insert(begin_word.clone());
        let mut found = false;

        while !current_level.is_empty() && !found {
            // remove all words used in this level only after the level starts,
            // so words within the same level cannot reference each other
            for w in &current_level {
                word_set.remove(w);
            }

            let mut next_level: HashSet<String> = HashSet::new();
            for word in &current_level {
                let mut chars: Vec<char> = word.chars().collect();
                for i in 0..chars.len() {
                    let original = chars[i];
                    for c in b'a'..=b'z' {
                        let c = c as char;
                        if c == original {
                            continue;
                        }
                        chars[i] = c;
                        let candidate: String = chars.iter().collect();
                        if word_set.contains(&candidate) {
                            next_level.insert(candidate.clone());
                            parents
                                .entry(candidate.clone())
                                .or_insert_with(Vec::new)
                                .push(word.clone());
                            if candidate == end_word {
                                found = true;
                            }
                        }
                    }
                    chars[i] = original; // restore before next position
                }
            }
            current_level = next_level;
        }

        if !found {
            return vec![];
        }

        // reconstruct all shortest paths by backtracking from endWord to beginWord
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut path: Vec<String> = vec![end_word.clone()];
        Self::backtrack(&end_word, &begin_word, &parents, &mut path, &mut result);
        result
    }

    fn backtrack(
        word: &String,
        begin_word: &String,
        parents: &HashMap<String, Vec<String>>,
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if word == begin_word {
            let mut full: Vec<String> = path.clone();
            full.reverse();
            result.push(full);
            return;
        }
        if let Some(ps) = parents.get(word) {
            for p in ps {
                path.push(p.clone());
                Self::backtrack(p, begin_word, parents, path, result);
                path.pop();
            }
        }
    }
}


