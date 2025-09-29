struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x < 0 { -1 } else { 1 };
        let num = x
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0);
        return sign * num;
    }

    // fn transform_int_to_vec_of_digits(x: i32) -> [i32; 31] {
    //     let mut digits: [i32; 31] = [0; 31];
    //     let mut remaining = x;
    //     for i in 0..digits.len() {
    //         let digit = remaining % 10;
    //         digits[i] = digit;
    //         remaining = (remaining - digit) / 10;
    //     }
    //     return digits;
    // }
    //
    // fn get_real_digit_count(digits: &[i32; 31]) -> usize {
    //     for i in 0..digits.len() {
    //         if digits[digits.len() - 1 - i] != 0 {
    //             return digits.len() - i;
    //         }
    //     }
    //     return 0;
    // }
    //
    // fn reverse_vec_of_digits(digits: &mut [i32; 31]) {
    //     let real_digit_count = Solution::get_real_digit_count(digits);
    //     for i in 0..real_digit_count / 2 {
    //         let to_swap_idx = i;
    //         let swap_with_idx = real_digit_count - 1 - i;
    //
    //         let to_swap = digits[to_swap_idx];
    //         let swap_with = digits[swap_with_idx];
    //
    //         digits[to_swap_idx] = swap_with;
    //         digits[swap_with_idx] = to_swap;
    //     }
    // }
    //
    // fn transform_vec_of_digits_to_int(digits: &[i32; 31]) -> i32 {
    //     let real_digit_count = Solution::get_real_digit_count(digits);
    //     let mut number = 0;
    //     for i in 0..real_digit_count {
    //         number += digits[i] * 10_i32.pow(i as u32);
    //     }
    //     return number;
    // }
}

#[cfg(test)]
mod tests {
    use crate::reverse_integer::Solution;

    #[test]
    fn case_1() {
        //Given
        let x = 123;
        let expexted = 321;

        //When
        let result = Solution::reverse(x);

        //Then
        assert_eq!(result, expexted);
    }

    #[test]
    fn case_2() {
        //Given
        let x = -123;
        let expexted = -321;

        //When
        let result = Solution::reverse(x);

        //Then
        assert_eq!(result, expexted);
    }

    #[test]
    fn case_3() {
        //Given
        let x = 120;
        let expexted = 21;

        //When
        let result = Solution::reverse(x);

        //Then
        assert_eq!(result, expexted);
    }

    #[test]
    fn case_4() {
        //Given
        let x = 2147483647;
        let expexted = 0;

        //When
        let result = Solution::reverse(x);

        //Then
        assert_eq!(result, expexted);
    }
}
