struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut rows = vec![String::new(); num_rows as usize];

        let mut curr_row = 0;
        let mut going_down = true;

        for c in s.chars() {
            rows[curr_row].push(c);

            if curr_row == 0 {
                going_down = true
            }
            if (curr_row + 1) == (num_rows as usize) {
                going_down = false
            }
            if going_down {
                curr_row += 1
            } else {
                curr_row -= 1
            };
        }

        return rows.join("");
    }
}

#[cfg(test)]
mod tests {
    use crate::zigzag::Solution;

    #[test]
    fn case_1() {
        //Given
        let s = String::from("PAYPALISHIRING");
        let num_rows = 3;
        let expected = String::from("PAHNAPLSIIGYIR");

        //When
        let result = Solution::convert(s, num_rows);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_2() {
        //Given
        let s = String::from("PAYPALISHIRING");
        let num_rows = 4;
        let expected = String::from("PINALSIGYAHRPI");

        //When
        let result = Solution::convert(s, num_rows);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_3() {
        //Given
        let s = String::from("A");
        let num_rows = 1;
        let expected = String::from("A");

        //When
        let result = Solution::convert(s, num_rows);

        //Then
        assert_eq!(result, expected);
    }
}
