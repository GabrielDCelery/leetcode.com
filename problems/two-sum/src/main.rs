use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, &current) in nums.iter().enumerate() {
            let expected = target - current;
            match m.get(&expected) {
                Some(&i2) => return vec![i as i32, i2 as i32],
                None => m.insert(current, i),
            };
        }
        panic!()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        // Given
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        // When
        let mut result = Solution::two_sum(nums, target);
        result.sort();

        // Then
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn case_2() {
        // Given
        let nums = vec![3, 2, 4];
        let target = 6;

        // When
        let mut result = Solution::two_sum(nums, target);
        result.sort();

        // Then
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn case_3() {
        // Given
        let nums = vec![3, 3];
        let target = 6;

        // When
        let mut result = Solution::two_sum(nums, target);
        result.sort();

        // Then
        assert_eq!(result, vec![0, 1]);
    }
}
