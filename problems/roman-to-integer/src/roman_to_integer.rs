use std::panic;

pub struct Solution {}

impl Solution {
    fn to_int(s: char) -> i32 {
        match s {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Could not find value for {s}"),
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut total: i32 = 0;

        let mut it = s.chars().peekable();

        loop {
            if let Some(curr) = it.next() {
                if let Some(next) = it.peek() {
                    let curr_int = Solution::to_int(curr);
                    let next_int = Solution::to_int(*next);
                    if curr_int < next_int {
                        total = total - curr_int + next_int;
                        it.next();
                        continue;
                    }
                }
                total += Solution::to_int(curr);
                continue;
            }
            break;
        }

        return total;
    }
}

mod tests {
    use crate::roman_to_integer::Solution;

    #[test]
    fn case_1() {
        // Given
        let case = String::from("III");
        let expected = 3;

        // When
        let result = Solution::roman_to_int(case);

        // Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_2() {
        // Given
        let case = String::from("LVIII");
        let expected = 58;

        // When
        let result = Solution::roman_to_int(case);

        // Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_3() {
        // Given
        let case = String::from("MCMXCIV");
        let expected = 1994;

        // When
        let result = Solution::roman_to_int(case);

        // Then
        assert_eq!(result, expected);
    }
}
