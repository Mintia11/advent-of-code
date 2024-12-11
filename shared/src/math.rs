use crate::num_traits::Num;

#[must_use]
pub fn uint<T: AsRef<str>>(s: T) -> usize {
    s.as_ref().parse().unwrap()
}

#[must_use]
pub fn uint_char(c: char) -> usize {
    (c as u8 - b'0').into()
}

#[must_use]
pub fn dist<T: Num>(a: T, b: T) -> T::Unsigned {
    a.abs_sub(&b)
}

pub fn two_dimensional_get<T, U>(array: &[T], x: usize, y: usize) -> Option<U>
where
    T: AsRef<[U]>,
    U: Clone,
{
    array.get(y).and_then(|row| row.as_ref().get(x).cloned())
}

pub fn two_dimensional_find<T, U>(array: &[T], to_find: U) -> Option<(usize, usize)>
where
    T: AsRef<[U]>,
    U: PartialEq<U> + Copy,
{
    array.iter().enumerate().find_map(|(y, val)| {
        let x = val
            .as_ref()
            .iter()
            .enumerate()
            .find_map(|(x, &val)| (val == to_find).then_some(x));

        if let Some(x) = x {
            Some((x, y))
        } else {
            None
        }
    })
}

pub const fn const_rotate_matrix<const SIZE: usize, T: Copy>(
    matrix: &[[T; SIZE]; SIZE],
    default: T,
) -> [[T; SIZE]; SIZE] {
    let mut new = [[default; SIZE]; SIZE];

    let mut i = 0;
    while i < SIZE {
        let mut j = 0;
        while j < SIZE {
            new[j][SIZE - 1 - i] = matrix[i][j];

            j += 1;
        }

        i += 1;
    }

    new
}
