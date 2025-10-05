struct Cell {
    candidates: Vec<u8>,
    solution: u8,
}

struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(board: &mut Vec<Vec<char>>) -> Self {
        let mut cells = Vec::new();
        for row in 0..9 {
            let mut cell_row = Vec::new();
            for col in 0..9 {
                let cell = if board[row][col] == '.' {
                    Cell {
                        candidates: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                        solution: 0,
                    }
                } else {
                    Cell {
                        candidates: vec![],
                        solution: board[row][col].to_digit(10).unwrap() as u8,
                    }
                };
                cell_row.push(cell);
            }
            cells.push(cell_row);
        }
        Board { cells }
    }

    fn solve(&mut self) {
        loop {
            let hashed_before_cycle = self.hash();
            for row_idx in 0..9 {
                for col_idx in 0..9 {
                    if self.is_cell_solved(row_idx, col_idx) {
                        continue;
                    }

                    self.eliminate(row_idx, col_idx, self.get_all_solved_for_row(row_idx));
                    self.eliminate(row_idx, col_idx, self.get_all_solved_for_col(col_idx));
                    self.eliminate(
                        row_idx,
                        col_idx,
                        self.get_all_solved_for_box(row_idx, col_idx),
                    );

                    self.set_cell_to_solved_if_can(row_idx, col_idx);
                }
            }
            let hashed_after_cycle = self.hash();
            if hashed_before_cycle == hashed_after_cycle {
                break;
            }
        }
    }

    fn is_cell_solved(&self, row_idx: usize, col_idx: usize) -> bool {
        let cell = &self.cells[row_idx][col_idx];
        return cell.solution != 0;
    }

    fn set_cell_to_solved_if_can(&mut self, row_idx: usize, col_idx: usize) -> bool {
        let cell = &mut self.cells[row_idx][col_idx];
        if cell.candidates.len() > 1 {
            return false;
        }
        cell.solution = cell.candidates[0];
        return true;
    }

    fn eliminate(&mut self, row_idx: usize, col_idx: usize, nums_to_eliminate: Vec<u8>) {
        let cell = &mut self.cells[row_idx][col_idx];
        let new_candidates: Vec<u8> = cell
            .candidates
            .iter()
            .filter(|&candidate| !nums_to_eliminate.contains(candidate))
            .copied()
            .collect();
        cell.candidates = new_candidates;
    }

    fn hash(&self) -> String {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| x.solution.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("")
    }

    fn get_all_solved_for_row(&self, row_idx: usize) -> Vec<u8> {
        let mut solved = vec![];
        for cell in &self.cells[row_idx] {
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
            let cell = &row[col_idx];
            if cell.solution == 0 {
                continue;
            }
            solved.push(cell.solution);
        }
        return solved;
    }

    fn get_all_solved_for_box(&self, row_idx: usize, col_idx: usize) -> Vec<u8> {
        let mut solved = vec![];
        let box_start_row = (row_idx / 3) * 3;
        let box_start_col = (col_idx / 3) * 3;
        for x in 0..3 {
            for y in 0..3 {
                let cell = &self.cells[box_start_row + x][box_start_col + y];
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
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut sudoku_board = Board::new(board);
        sudoku_board.solve();

        // Update the original board with the solved values
        for row_idx in 0..9 {
            for col_idx in 0..9 {
                if sudoku_board.cells[row_idx][col_idx].solution != 0 {
                    board[row_idx][col_idx] =
                        (sudoku_board.cells[row_idx][col_idx].solution + b'0') as char;
                }
            }
        }
    }
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
