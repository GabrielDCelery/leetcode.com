use std::{collections::HashMap, panic};

pub struct Solution {}

impl Solution {
    fn get_value_of_roman(s: &str) -> i32 {
        let m: HashMap<&str, i32> = HashMap::from([
            ("I", 1),
            ("V", 5),
            ("X", 10),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000),
        ]);

        match m.get(s) {
            Some(v) => *v,
            None => panic!("Could not find value for {s}"),
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut pointer: usize = 0;
        let mut total: i32 = 0;

        while pointer < s.len() {
            if pointer == s.len() - 1 {
                total += Solution::get_value_of_roman(&s[pointer..(pointer + 1)]);
                break;
            }

            let curr = Solution::get_value_of_roman(&s[pointer..(pointer + 1)]);
            let next = Solution::get_value_of_roman(&s[(pointer + 1)..(pointer + 2)]);

            if curr < next {
                total -= curr;
                total += next;
                pointer += 2;
                continue;
            }

            total += curr;
            pointer += 1;
            continue;
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
