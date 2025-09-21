# The problem

[Two Sum](https://leetcode.com/problems/two-sum/)

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

# Notes

Creating a hashmap using the `with_capacity` rather than `new` is an interesting debate.

```rust
let mut m: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
```

The match statement can be neatly used to return the final solution

```rust
for (i, &current) in nums.iter().enumerate() {
    let expected = target - current;
    match m.get(&expected) {
        Some(&i2) => return vec![i as i32, i2 as i32],
        None => m.insert(current, i),
    };
}
```
