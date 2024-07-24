// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut last_seen: HashMap<char, i32> = HashMap::new();
    s.chars()
        .enumerate()
        .fold((0, -1), |(max, start), (idx, c)| {
            let new_start = last_seen
                .insert(c, idx as i32)
                .unwrap_or(-1)
                .max(start);
            (max.max((idx as i32) - new_start), new_start)
        })
        .0
}

#[cfg(test)]
mod test_longest_substring_without_repeating_characters {
    use super::*;

    fn case(s: &str, expected: i32) {
        let actual = length_of_longest_substring(s.to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_case_1() {
        case("abcabcbb", 3);
    }

    #[test]
    fn test_case_2() {
        case("bbbbb", 1);
    }

    #[test]
    fn test_case_3() {
        case("pwwkew", 3);
    }
}
