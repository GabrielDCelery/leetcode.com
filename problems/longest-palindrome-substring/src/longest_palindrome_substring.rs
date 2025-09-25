struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest_slice_idxs = [0, 0];
        let letters: Vec<char> = s.chars().collect();
        for i in 0..letters.len() {
            for j in 0..letters.len() {
                // if j is less than i then we already covered that scenario when i and j were
                // swapped
                if j <= i {
                    continue;
                }
                // if the current index pair we are looking at is shorter than the longest pair
                // then even if it is a palindrome it won't be longer
                let longest_length = longest_slice_idxs[1] - longest_slice_idxs[0];
                let curr_length = j - i;
                if curr_length <= longest_length {
                    continue;
                }
                // check if actually palindrome
                if !Solution::is_segment_palindrome(&letters, i, j) {
                    continue;
                }
                // if palindrome replace with the longer option
                longest_slice_idxs = [i, j];
            }
        }
        // convert slice of letters back to a string
        return letters[longest_slice_idxs[0]..=longest_slice_idxs[1]]
            .into_iter()
            .collect();
    }

    fn is_segment_palindrome(chars: &Vec<char>, i: usize, j: usize) -> bool {
        let mut left = i;
        let mut right = j;

        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_palindrome_substring::Solution;

    #[test]
    fn case_1() {
        //Given
        let s = String::from("babad");
        let expected = String::from("bab");

        //When
        let soltuion = Solution::longest_palindrome(s);

        //Then
        assert_eq!(soltuion, expected);
    }

    #[test]
    fn case_2() {
        //Given
        let s = String::from("cbbd");
        let expected = String::from("bb");

        //When
        let soltuion = Solution::longest_palindrome(s);

        //Then
        assert_eq!(soltuion, expected);
    }
}
