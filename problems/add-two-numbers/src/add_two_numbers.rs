#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1_as_num = Solution::ll_to_int(&l1, 0, None);
        let l2_as_num = Solution::ll_to_int(&l2, 0, None);
        let target_number = l1_as_num + l2_as_num;
        Solution::int_to_ll(target_number)
    }

    fn ll_to_int(ll: &Option<Box<ListNode>>, mut acc: i32, level: Option<u32>) -> i32 {
        if let Some(node) = ll {
            let curr_level = level.unwrap_or(0);
            acc += node.val * 10_i32.pow(curr_level);
            let next_level = curr_level + 1;
            Solution::ll_to_int(&node.next, acc, Some(next_level))
        } else {
            acc
        }
    }

    fn int_to_ll(num: i32) -> Option<Box<ListNode>> {
        let digit = num % 10;
        let mut curr_node = Box::new(ListNode::new(digit));
        let remaining = (num - digit) / 10;
        if remaining != 0 {
            curr_node.next = Solution::int_to_ll(remaining);
        }
        Some(curr_node)
    }
}

#[cfg(test)]
mod tests {
    use crate::add_two_numbers::{ListNode, Solution};

    #[test]
    fn convert_single_digit_to_ll() {
        // Given
        let num = 9;
        let expected = Some(Box::new(ListNode::new(9)));

        // When
        let result = Solution::int_to_ll(num);

        // Then
        assert_eq!(result, expected);
    }

    #[test]
    fn case_1() {
        // Given
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));

        // When
        let result = Solution::add_two_numbers(l1, l2);

        // Then
        assert_eq!(result, expected);
    }
}
