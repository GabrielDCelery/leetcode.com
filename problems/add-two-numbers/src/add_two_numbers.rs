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
        return Solution::combine_lls(l1, l2, 0);
    }

    fn combine_lls(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carriover: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(l1), Some(l2)) => {
                let total = l1.val + l2.val + carriover;
                let node_value = total % 10;
                let next_carriover = if total > node_value { 1 } else { 0 };
                Some(Box::new(ListNode {
                    val: node_value,
                    next: Solution::combine_lls(l1.next, l2.next, next_carriover),
                }))
            }
            (Some(l1), None) => {
                let total = l1.val + carriover;
                let node_value = total % 10;
                let next_carriover = if total > node_value { 1 } else { 0 };
                Some(Box::new(ListNode {
                    val: node_value,
                    next: Solution::combine_lls(l1.next, None, next_carriover),
                }))
            }
            (None, Some(l2)) => {
                let total = l2.val + carriover;
                let node_value = total % 10;
                let next_carriover = if total > node_value { 1 } else { 0 };
                Some(Box::new(ListNode {
                    val: node_value,
                    next: Solution::combine_lls(None, l2.next, next_carriover),
                }))
            }
            (None, None) => {
                if carriover > 0 {
                    Some(Box::new(ListNode {
                        val: carriover,
                        next: None,
                    }))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::add_two_numbers::{ListNode, Solution};

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
