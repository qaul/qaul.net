//! An iterator over a slice that uses another iterator to control the next element in
//! the sequence.

use std::marker::PhantomData;

/// An iterator over a slice that uses another iterator to control the next element in the
/// sequence, in any arbitrary order, without copying.
///
/// # Panics
/// Panics if the control iterator presents an index outside the bounds of the
/// data structure being controlled.
///
/// # Example
/// ```
/// # use permute::arbitrary_tandem_control_iter::ArbitraryTandemControlIterator;
/// let data = vec![
///     String::from("red"),
///     String::from("orange"),
///     String::from("yellow"),
///     String::from("green"),
///     String::from("blue"),
///     String::from("indigo"),
///     String::from("violet"),
/// ];
///
/// let control = vec![6, 5, 4, 3, 2, 1];
///
///    let atci = ArbitraryTandemControlIterator::new(&data, control.clone().into_iter());
///
///    for (atci_val, rev_val) in atci.zip(data.iter().rev()) {
///        // Critically, these are both &std::string::String. No copying occurred.
///        assert_eq!(atci_val, rev_val);
///    }
/// ```
pub struct ArbitraryTandemControlIterator<'a, T, C: Iterator<Item = usize>> {
    data: &'a [T],
    control: C,
    _pd: PhantomData<&'a T>,
}

impl<'a, T, C: Iterator<Item = usize>> Iterator for ArbitraryTandemControlIterator<'a, T, C> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.control.next() {
            Some(index) => Some(&self.data[index]),
            None => None,
        }
    }
}

impl<'a, T, C: Iterator<Item = usize>> ArbitraryTandemControlIterator<'a, T, C> {
    pub fn new(data: &'a [T], control: C) -> Self {
        Self {
            data,
            control,
            _pd: PhantomData,
        }
    }
}

#[test]
fn homogeneous_vecs() {
    let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let control = vec![1, 1, 2, 3, 3, 2, 1];
    let atci = ArbitraryTandemControlIterator::new(&data, control.clone().into_iter());
    let output: Vec<_> = atci.cloned().collect();
    assert_eq!(output, control);
}

#[test]
fn heterogeneous_vecs_nocopy() {
    let data = vec![
        String::from("red"),
        String::from("orange"),
        String::from("yellow"),
        String::from("green"),
        String::from("blue"),
        String::from("indigo"),
        String::from("violet"),
    ];

    let control = vec![6, 5, 4, 3, 2, 1];

    let atci = ArbitraryTandemControlIterator::new(&data, control.clone().into_iter());

    for (atci_val, rev_val) in atci.zip(data.iter().rev()) {
        // Critically, these are both &std::string::String. No copying occurred.
        assert_eq!(atci_val, rev_val);
    }
}
