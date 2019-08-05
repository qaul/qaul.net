//! Combining ATCI and permute() to create efficient permutations
use crate::arbitrary_tandem_control_iter::ArbitraryTandemControlIterator;
use crate::permutations::permute_owned;

/// Returns an iterator of iterators, with all the permutations of the given slice,
/// without copying that data.
pub fn permutations_of<'a, T: Clone + Sized>(
    items: &'a [T],
) -> impl Iterator<Item = impl Iterator<Item = &'a T>> {
    let indices: Vec<usize> = (0..items.len()).collect();
    let permutations: Vec<Vec<usize>> = permute_owned(indices);

    let mut atcis: Vec<_> = Vec::with_capacity(permutations.len());
    for permutation in permutations.into_iter() {
        atcis.push(ArbitraryTandemControlIterator::new(
            items,
            permutation.into_iter(),
        ));
    }
    atcis.into_iter()
}

#[test]
fn strings_nocopy() {
    let data = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    let mut permutations: Vec<Vec<&String>> = Vec::new();
    for permutation_iter in permutations_of(&data) {
        permutations.push(permutation_iter.collect());
    }
    assert_eq!(
        vec![
            vec!["red", "green", "blue"],
            vec!["green", "red", "blue"],
            vec!["blue", "red", "green"],
            vec!["red", "blue", "green"],
            vec!["green", "blue", "red"],
            vec!["blue", "green", "red"]
        ],
        permutations
    );
}
