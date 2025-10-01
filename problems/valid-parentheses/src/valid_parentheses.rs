struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => panic!("Unhandled character {c}"),
            }
        }

        return stack.len() == 0;
    }
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

    #[test]
    fn case_6() {
        //Given
        let s = String::from("]");
        let expected = false;

        //When
        let result = Solution::is_valid(s);

        //Then
        assert_eq!(result, expected);
    }
}
