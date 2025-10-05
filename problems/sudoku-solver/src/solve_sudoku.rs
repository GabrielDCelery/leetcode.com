struct Cell {
    candidates: Vec<u8>,
    solution: u8,
}

struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn get_all_solved_for_row(&self, row_idx: usize) -> Vec<u8> {
        let mut solved = vec![];
        for cell in self.cells.get(row_idx).unwrap() {
            if cell.solution == 0 {
                continue;
            }
            solved.push(cell.solution);
        }
        return solved;
    }

    fn get_all_solved_for_col(&self, col_idx: usize) -> Vec<u8> {
        let mut solved = vec![];
        for row in self.cells.iter() {
            let cell = row.get(col_idx).unwrap();
            if cell.solution == 0 {
                continue;
            }
            solved.push(cell.solution);
        }
        return solved;
    }

    fn get_all_solved_for_box(&self, row_idx: usize, col_idx: usize) -> Vec<u8> {
        let mut solved = vec![];
        let offset_x = row_idx / 3;
        let offset_y = col_idx / 3;
        for y in 0..3 {
            for x in 0..3 {
                let row = self.cells.get(offset_x + x).unwrap();
                let cell = row.get(offset_y + y).unwrap();
                if cell.solution == 0 {
                    continue;
                }
                solved.push(cell.solution);
            }
        }
        return solved;
    }
}

struct Solution {}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {}
}

#[cfg(test)]
mod tests {
    use crate::solve_sudoku::Solution;

    #[test]
    fn case_1() {
        //Given
        let mut board = vec![
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
        Solution::solve_sudoku(&mut board);

        //Then
        assert_eq!(
            board,
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ]
        );
    }
}
