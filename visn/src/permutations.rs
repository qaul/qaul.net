//! Tools for generating permutations using Heap's algorithm

fn heaps_inner<T: Clone>(k: usize, values: &mut [T]) -> Vec<Vec<T>> {
    let mut v = Vec::new();
    if k <= 1 {
        v.push(values.iter().cloned().collect());
    } else {
        v.append(&mut heaps_inner(k - 1, values));
        for i in 0..(k - 1) {
            if k % 2 == 0 {
                values.swap(i, k - 1);
            } else {
                values.swap(0, k - 1);
            }
            v.append(&mut heaps_inner(k - 1, values));
        }
    }

    return v;
}

pub fn permute<T: Clone>(values: &[T]) -> Vec<Vec<T>> {
    let mut values: Vec<T> = values.iter().cloned().collect();
    permute_owned(values)
}

pub fn permute_owned<T: Clone>(values: Vec<T>) -> Vec<Vec<T>> {
    let mut values = values;
    heaps_inner(values.len(), &mut values)
}

#[test]
fn permute_numbers() {
    assert_eq!(
        permute(&[1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![2, 1, 3],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 3, 1],
            vec![3, 2, 1]
        ]
    );
}
