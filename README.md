# swizzle

simd swizzle for arrays.

```rust
use swizzle::array;
assert_eq!(
    [10, 11, 12, 13, 14, 15].swizzle([3, 0, 1, 2, 5]),
    [13, 10, 11, 12, 15]
);
```
