#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_map = HashMap::new();
        let (mut left, mut max_len) = (0, 0);

        for (right, ch) in s.chars().enumerate() {
            if let Some(&last_index) = char_map.get(&ch) {
                left = left.max(last_index + 1);
            }

            char_map.insert(ch, right);
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}

#[test]
fn test_length_of_longest_substring() {
    let str = "abcabcbb".to_string();
    let result = Solution::length_of_longest_substring(str);
    assert_eq!(result, 3);

    let str = "bbbbb".to_string();
    let result = Solution::length_of_longest_substring(str);
    assert_eq!(result, 1);

    let str = "pwwkew".to_string();
    let result = Solution::length_of_longest_substring(str);
    assert_eq!(result, 3);
}