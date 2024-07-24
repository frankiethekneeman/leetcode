/*
 * https://leetcode.com/problems/add-two-numbers/
 *
 * First: Definitions directly from leetcode.
 */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: u32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: u32) -> Self {
        ListNode { next: None, val }
    }
}
/*
 * Solution:
 */
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    full_adder(l1, l2, 0)
}

fn full_adder(
    left: Option<Box<ListNode>>,
    right: Option<Box<ListNode>>,
    carry: u32,
) -> Option<Box<ListNode>> {
    if left.is_none() && right.is_none() && carry == 0 {
        return None;
    }
    let l = left.as_ref().map_or(0, |b| b.val);
    let r = right.as_ref().map_or(0, |b| b.val);
    let result = l + r + carry;

    let mut to_return = ListNode::new(result % 10);
    to_return.next = full_adder(
        left.and_then(|b| b.next),
        right.and_then(|b| b.next),
        result / 10,
    );
    Some(Box::new(to_return))
}
#[cfg(test)]
mod test_add_two_numbers {
    use super::*;

    fn to_list_rec(n: u32) -> Option<Box<ListNode>> {
        if n == 0 {
            return None;
        }
        let mut to_return = ListNode::new(n % 10);
        to_return.next = to_list_rec(n / 10);
        Some(Box::new(to_return))
    }

    fn to_list(n: u32) -> Option<Box<ListNode>> {
        to_list_rec(n).or(Some(Box::new(ListNode::new(n))))
    }

    fn case(lhs: u32, rhs: u32) {
        let l1 = to_list(lhs);
        let l2 = to_list(rhs);
        let expected = to_list(lhs + rhs);
        let actual = add_two_numbers(l1, l2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_case_1() {
        case(342, 564)
    }

    #[test]
    fn test_case_2() {
        case(0, 0)
    }

    #[test]
    fn test_case_3() {
        case(9999999, 9999)
    }
}
