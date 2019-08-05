//! An iterator that uses another iterator over usize
//! to control what the next element is.

use std::marker::PhantomData;
use std::ops::Index;

/// An iterator that uses another iterator to control the next element in the iteration,
/// in any arbitrary order.
///
/// # Limitations
/// Currently, the control iterator must be `Item = usize`.
///
/// # Panics
/// Panics if the control iterator asks to return an index outside the bounds of the
/// data structure being controlled.
pub struct ArbitraryTandemControlIterator<'a, T, I: Index<usize, Output = T>, C: Iterator<Item = usize>>
{
    data: &'a I,
    control: C,
    _pd: PhantomData<&'a T>,
}

impl<'a, T, I: Index<usize, Output = T>, C: Iterator<Item = usize>> Iterator
    for ArbitraryTandemControlIterator<'a, T, I, C>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.control.next() {
            Some(index) => Some(&self.data[index]),
            None => None,
        }
    }
}

impl<'a, T, I: Index<usize, Output = T>, C: Iterator<Item = usize>>
    ArbitraryTandemControlIterator<'a, T, I, C>
{
    pub fn new(data: &'a I, control: C) -> Self {
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
    let mut atci = ArbitraryTandemControlIterator::new(&data, control.clone().into_iter());
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

    let mut atci = ArbitraryTandemControlIterator::new(&data, control.clone().into_iter());

    for (atci_val, rev_val) in atci.zip(data.iter().rev()) {
        // Critically, these are both &std::string::String. No copying occurred.
        assert_eq!(atci_val, rev_val);
    }
}
