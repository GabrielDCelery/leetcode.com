struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {}
}

#[cfg(test)]
mod tests {
    use crate::valid_parentheses::Solution;

    #[test]
    fn case_1() {
        //Given
        let s = String::from("()");
        let expected = true;

        //When
        let result = Solution::is_valid(s);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_2() {
        //Given
        let s = String::from("()[]{}");
        let expected = true;

        //When
        let result = Solution::is_valid(s);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_3() {
        //Given
        let s = String::from("(]");
        let expected = false;

        //When
        let result = Solution::is_valid(s);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_4() {
        //Given
        let s = String::from("([])");
        let expected = true;

        //When
        let result = Solution::is_valid(s);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_5() {
        //Given
        let s = String::from("([)]");
        let expected = false;

        //When
        let result = Solution::is_valid(s);

        //Then
        assert_eq!(result, expected);
    }
}
