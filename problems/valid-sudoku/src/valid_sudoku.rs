use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        return Solution::are_rows_valid(&board)
            && Solution::are_columns_valid(&board)
            && Solution::are_sub_boxes_valid(&board);
    }

    fn are_rows_valid(board: &Vec<Vec<char>>) -> bool {
        for row in board {
            let mut seen = HashSet::new();
            for &cell in row {
                if cell != '.' && !seen.insert(cell) {
                    return false;
                }
            }
        }
        true
    }

    fn are_columns_valid(board: &Vec<Vec<char>>) -> bool {
        for col_idx in 0..9 {
            let mut seen = HashSet::new();
            for row in board {
                let cell = row[col_idx];
                if cell != '.' && !seen.insert(cell) {
                    return false;
                }
            }
        }
        true
    }

    fn are_sub_boxes_valid(board: &Vec<Vec<char>>) -> bool {
        for offset_y in [0, 3, 6] {
            for offset_x in [0, 3, 6] {
                let mut seen = HashSet::new();
                for y in 0..3 {
                    for x in 0..3 {
                        let cell = board[y + offset_y][x + offset_x];
                        if cell != '.' && !seen.insert(cell) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::valid_sudoku::Solution;

    #[test]
    fn case_1() {
        //Given
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        //When
        let result = Solution::is_valid_sudoku(board);

        //Then
        assert_eq!(result, true);
    }

    #[test]
    fn case_2() {
        //Given
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        //When
        let result = Solution::is_valid_sudoku(board);

        //Then
        assert_eq!(result, false);
    }
}
