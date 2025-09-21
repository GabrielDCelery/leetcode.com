struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }

        strs.into_iter()
            .reduce(|acc, x| {
                let mut common = String::new();

                return common;
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_common::Solution;

    #[test]
    fn case_1() {
        // Given
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let expected = "fl";

        // When
        let result = Solution::longest_common_prefix(strs);

        // Then
        assert_eq!(result, expected)
    }

    #[test]
    fn case_2() {
        // Given
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let expected = "";

        // When
        let result = Solution::longest_common_prefix(strs);

        // Then
        assert_eq!(result, expected)
    }

    #[test]
    fn case_3() {
        // Given
        let strs = vec!["cir".to_string(), "car".to_string()];
        let expected = "c";

        // When
        let result = Solution::longest_common_prefix(strs);

        // Then
        assert_eq!(result, expected)
    }

    #[test]
    fn case_4() {
        // Given
        let strs = vec!["a".to_string()];
        let expected = "a";

        // When
        let result = Solution::longest_common_prefix(strs);

        // Then
        assert_eq!(result, expected)
    }

    #[test]
    fn case_5() {
        // Given
        let strs = vec!["".to_string()];
        let expected = "";

        // When
        let result = Solution::longest_common_prefix(strs);

        // Then
        assert_eq!(result, expected)
    }
}
