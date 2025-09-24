struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut shortest = strs.get(0).unwrap_or(&String::new()).clone();

        for current in strs.iter() {
            shortest = Solution::longest_common_for_two_words(&shortest, current);
        }

        return shortest;
    }

    fn longest_common_for_two_words(first: &String, second: &String) -> String {
        let mut shortest = String::new();
        for (c1, c2) in first.chars().zip(second.chars()) {
            if c1 == c2 {
                shortest.push_str(c1.to_string().as_str());
            } else {
                break;
            }
        }
        return shortest;
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
