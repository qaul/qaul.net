//! Combining ATCI and permute() to create efficient permutations
use crate::arbitrary_tandem_control_iter::ArbitraryTandemControlIterator;
use crate::permutations::permute;

/// Produce an iterator over iterators, each one of which yields one permutation of the
/// provided slice. No copying of elements of the slice occurs.
///
/// # Complexity
///
/// This function is `O(n!)` in both space and time, though with a lower constant than
/// `permute` for collections with large elements, since it does not copy elements of the
/// give slice, only indices.
///
/// # Determinism
///
/// The order of the permutations is deterministic and can be found ahead of time by
/// consulting the OEIS sequence for reverse colexicographic ordering, using
/// the appropriate elements of [A280318](https://oeis.org/A280318) as indices into
/// [A055089](https://oeis.org/A055089).
///
/// # Example
///
/// For instance, printing all the permutations of the sequence
/// `["red", "green", "blue"]`:
/// ```
/// # use permute::permutations_of;
/// for permutation in permutations_of(&["red", "green", "blue"]) {
///     for element in permutation {
///         print!("{}, ", element);
///     }
///     println!("");
/// }
/// ```
///
/// Based on the ordering provided by Heap's algorithm, it's guaranteed that this
/// program will produce:
///
/// ```text
/// red, green, blue,
/// green, red, blue,
/// blue, red, green,
/// red, blue, green,
/// green, blue, red,
/// blue, green, red,
/// ```
pub fn permutations_of<'a, T: Clone + Sized>(
    items: &'a [T],
) -> impl Iterator<Item = impl Iterator<Item = &'a T>> {
    let indices: Vec<usize> = (0..items.len()).collect();
    let permutations: Vec<Vec<usize>> = permute(indices);

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
