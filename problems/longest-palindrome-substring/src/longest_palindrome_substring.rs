struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        return String::new();
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_palindrome_substring::Solution;

    #[test]
    fn case_1() {
        //Given
        let s = String::from("babad");
        let expected = String::from("bab");

        //When
        let soltuion = Solution::longest_palindrome(s);

        //Then
        assert_eq!(soltuion, expected);
    }

    #[test]
    fn case_2() {
        //Given
        let s = String::from("cbbd");
        let expected = String::from("bb");

        //When
        let soltuion = Solution::longest_palindrome(s);

        //Then
        assert_eq!(soltuion, expected);
    }
}
