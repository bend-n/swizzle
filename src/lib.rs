#[allow(non_camel_case_types)]
/// Swizzle an array. The result is `a[index[i]]`.
pub trait array<T> {
    /// Provides a method to arrays that selects specific indexes.
    /// ```
    /// use swizzle::array;
    /// assert_eq!(
    ///   [10, 11, 12, 13, 14, 15].swizzle([3, 0, 1, 2, 5]),
    ///   [13, 10, 11, 12, 15]
    /// );
    /// ```
    /// The result is `self[indexes[i]]`
    fn swizzle<const N: usize>(self, indexes: [usize; N]) -> [T; N];
}

impl<T: Clone, const N_: usize> array<T> for [T; N_] {
    fn swizzle<const N: usize>(self, indexes: [usize; N]) -> [T; N] {
        std::array::from_fn(|i| self[indexes[i]].clone())
    }
}

#[macro_export]
/// Swizzling for tuples. I think arrays are better but i added this anyways.
/// ```
/// assert_eq!(
///   swizzle::tuple!((10u8, 11i8, 12i32, 13u32, 14i64, 15u64), [3, 0, 1, 2, 5]),
///   (13, 10, 11, 12, 15),
/// );
/// ```
macro_rules! tuple {
    ($x:expr, [$($index:tt),* $(,)?]) => {{
        let t = $x;
        ($(t.$index,)*)
    }};
}
