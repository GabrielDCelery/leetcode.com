struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let num_sym_pairs = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut solution = String::new();
        let mut remaining = num;

        while remaining > 0 {
            for pair in num_sym_pairs.iter() {
                if remaining >= pair.0 {
                    solution += pair.1;
                    remaining -= pair.0;
                    break;
                }
            }
        }

        return solution;
    }
}

#[cfg(test)]
mod tests {
    use crate::integer_to_roman::Solution;

    #[test]
    fn case_1() {
        // Given
        let num = 3749;
        let expected = String::from("MMMDCCXLIX");

        // When
        let result = Solution::int_to_roman(num);

        // Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_2() {
        // Given
        let num = 58;
        let expected = String::from("LVIII");

        // When
        let result = Solution::int_to_roman(num);

        // Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_3() {
        // Given
        let num = 1994;
        let expected = String::from("MCMXCIV");

        // When
        let result = Solution::int_to_roman(num);

        // Then
        assert_eq!(result, expected);
    }
}
