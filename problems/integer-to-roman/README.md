# The problem

[Integer to Roman](https://leetcode.com/problems/integer-to-roman/description/)

Given an integer, convert it to a Roman numeral.

# Notes

The main challange was actually how to concatenate strings in rust.

```rust
solution = format!("{}{}", solution, pair.1);
// Solution is a String
// pair.1 is a &str
```
