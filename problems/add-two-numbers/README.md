# The problem

[Add two numbers](https://leetcode.com/problems/add-two-numbers/description/)

You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

# Notes

Built it with a recursive algorithm and match operator.

If I get a chance would like to investigate if

A. there is a better solution than match
B. Could I use references instead of passing ownership to recursive function

Originally tried to pass in references but that was challenging, so ended up using the `move` feature of rust, but did ask AI to give me a writeup on how it works to make sure I better understand it.

In Rust, when you pass a value to a function, ownership is **moved** unless the parameter is a reference (`&T`) or the type implements `Copy`. In your case, `Option<Box<ListNode>>` does **not** implement `Copy`, so ownership is moved on each function call.

## Ownership Flow Through the Recursion

Let's trace through what happens:

### 1. Initial Call

```rust
Solution::combine_lls(l1, l2, 0)
```

- `l1` and `l2` are **moved** into the function
- The original caller no longer owns these values

### 2. Pattern Matching

```rust
match (l1, l2) {
    (Some(l1), Some(l2)) => {
        // l1 and l2 are now owned Box<ListNode> values
```

- The `match` consumes the `Option<Box<ListNode>>` values
- Inside each arm, `l1` and `l2` become owned `Box<ListNode>` values

### 3. Accessing Fields

```rust
let total = l1.val + l2.val + carriover;
```

- When you access `l1.val`, Rust automatically dereferences the `Box`
- The `Box<ListNode>` is still owned by this scope

### 4. The Recursive Call

```rust
next: Solution::combine_lls(l1.next, l2.next, next_carriover)
```

This is where the magic happens:

- `l1.next` **moves** the `Option<Box<ListNode>>` out of `l1`
- `l2.next` **moves** the `Option<Box<ListNode>>` out of `l2`
- After this line, `l1` and `l2` are partially moved and can't be used again
- The recursive call takes ownership of these `next` values
