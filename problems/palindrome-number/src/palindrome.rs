pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x == 0 {
            return true;
        }
        if x < 0 {
            return false;
        }

        let digit_count = ((x as f64).log10().floor() as u32) + 1;

        let mut tracking_num = x;
        let mut reverted: i32 = 0;

        for i in 0..digit_count {
            let remainder = tracking_num % 10;
            reverted += remainder * 10_i32.pow(digit_count - i - 1);
            tracking_num = (tracking_num - remainder) / 10
        }

        return reverted == x;
    }
}

#[cfg(test)]
mod tests {
    use crate::palindrome::Solution;

    #[test]
    fn case_1() {
        //Given
        let number = 121;
        let expected = true;

        //When
        let result = Solution::is_palindrome(number);
        //Then

        assert_eq!(result, expected);
    }

    #[test]
    fn case_2() {
        //Given
        let number = -121;
        let expected = false;

        //When
        let result = Solution::is_palindrome(number);
        //Then

        assert_eq!(result, expected);
    }

    #[test]
    fn case_3() {
        //Given
        let number = 10;
        let expected = false;

        //When
        let result = Solution::is_palindrome(number);
        //Then

        assert_eq!(result, expected);
    }
}
