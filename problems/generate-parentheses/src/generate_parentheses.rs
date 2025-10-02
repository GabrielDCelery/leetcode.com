#[derive(Clone)]
struct ParenthesesVec {
    capacity: usize,
    chars: Vec<char>,
    open_count: usize,
    close_count: usize,
}

impl ParenthesesVec {
    fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity,
            chars: vec![],
            open_count: 0,
            close_count: 0,
        }
    }

    fn to_str(&self) -> String {
        return self.chars.iter().collect();
    }

    fn is_full(&self) -> bool {
        return self.capacity == self.chars.len();
    }

    fn add_opening(&mut self) {
        self.chars.push('(');
        self.open_count += 1;
    }

    fn add_closing(&mut self) {
        self.chars.push(')');
        self.close_count += 1;
    }
}

struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer: Vec<String> = vec![];
        Solution::build_answer(&mut answer, ParenthesesVec::new(2 * n as usize));
        return answer;
    }

    fn build_answer(answer: &mut Vec<String>, curr: ParenthesesVec) {
        if curr.is_full() {
            answer.push(curr.to_str());
            return;
        }

        if curr.open_count >= curr.close_count && curr.open_count < curr.capacity / 2 {
            let mut cloned = curr.clone();
            cloned.add_opening();
            Solution::build_answer(answer, cloned);
        }

        if curr.open_count > curr.close_count {
            let mut cloned = curr.clone();
            cloned.add_closing();
            Solution::build_answer(answer, cloned);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_parentheses::Solution;

    #[test]
    fn case_1() {
        //Given
        let n = 3;
        let expected = ["((()))", "(()())", "(())()", "()(())", "()()()"];

        //When
        let result = Solution::generate_parenthesis(n);

        //Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_2() {
        //Given
        let n = 1;
        let expected = ["()"];

        //When
        let result = Solution::generate_parenthesis(n);

        //Then
        assert_eq!(result, expected);
    }
}
