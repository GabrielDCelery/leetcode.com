struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_parentheses::Solution;

    #[test]
    fn case_1() {
        //Given
        let n = 3;
        let expected = ["((()))", "(()())", "(())()", "()(())", "()()()"];

        //When
        let result = Solution::generate_parenthesis(n);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_2() {
        //Given
        let n = 1;
        let expected = ["()"];

        //When
        let result = Solution::generate_parenthesis(n);

        //Then
        assert_eq!(result, expected);
    }
}
