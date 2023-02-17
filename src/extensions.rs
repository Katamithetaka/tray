pub trait IteratorExt: Iterator {
    /// Transforms an iterator into a collection.
    ///
    /// `collect_into_vec()` can take anything iterable that implements collect,
    /// and turn it into a relevant vector. It is a shorthand to `collect::<Vec<_>>()` which
    /// handles the type inference thanks to rust being awesome.
    ///
    /// The most basic pattern in which `collect()` is used is to turn one
    /// collection into another. You take a collection, call [`iter`] on it,
    /// do a bunch of transformations, and then `collect()` at the end.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let a = [1, 2, 3];
    ///
    /// let doubled = a.iter()
    ///                .map(|&x| x * 2)
    ///                .collect_into_vec();
    ///
    /// assert_eq!(vec![2, 4, 6], doubled);
    /// ```
    /// Which is equivalent equivalent to
    /// ```
    /// let doubled: Vec<i32> = a.iter()
    ///                         .map(|&x| x * 2)
    ///                         .collect();
    ///
    /// ```
    ///
    /// Note that because we use a shorthand, we don't need to specify that
    /// we are collecting into a vec as it is the only valid option.
    ///
    /// [`collect`]: Iterator::collect
    /// [`iter`]: Iterator::next
    /// [`char`]: type@char
    fn collect_into_vec(self) -> Vec<Self::Item>
    where
        Self: Sized,
        Vec<Self::Item>: FromIterator<Self::Item>,
    {
        return self.collect();
    }
}

impl<I, T: Iterator<Item = I>> IteratorExt for T {}
