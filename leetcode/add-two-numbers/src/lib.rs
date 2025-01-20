#![allow(dead_code)]

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

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;
        let mut carry = 0;

        let (mut l1, mut l2) = (l1, l2);

        while l1.is_some() || l2.is_some() || carry > 0 {
            let val1 = l1.as_ref().map_or(0, |node| node.val);
            let val2 = l2.as_ref().map_or(0, |node| node.val);

            let sum = carry + val1 + val2;

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        dummy_head.next
    }
}

#[test]
fn test_add_two_numbers() {
    fn create_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &num in nums.iter().rev() {
            let new_node = Box::new(ListNode {
                val: num,
                next: head,
            });
            head = Some(new_node);
        }
        head
    }

    let l1 = create_linked_list(vec![2, 4, 3]);
    let l2 = create_linked_list(vec![5, 6, 4]);
    let expected = create_linked_list(vec![7, 0, 8]);
    assert_eq!(Solution::add_two_numbers(l1, l2), expected);

    let l1 = create_linked_list(vec![0]);
    let l2 = create_linked_list(vec![0]);
    let expected = create_linked_list(vec![0]);
    assert_eq!(Solution::add_two_numbers(l1, l2), expected);

    let l1 = create_linked_list(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = create_linked_list(vec![9, 9, 9, 9]);
    let expected = create_linked_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
    assert_eq!(Solution::add_two_numbers(l1, l2), expected);
}
