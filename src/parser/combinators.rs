use std::ops::{Bound, RangeBounds};

use crate::parser::Parser;

#[macro_export]
macro_rules! all {
    ($parser:expr $(, $f:expr)+ $(,)?) => {{
        Some(
            (
                $(
                    $f($parser)?,
                )+
            )
        )
    }};
}

#[macro_export]
macro_rules! any {
    ($parser:expr $(, $f:expr)+ $(,)?) => {{
        $(
            match $f($parser) {
                Some(o) => return Some(o),
                None => (),
            }
        )+
        None
    }};
}

#[inline]
pub fn repeat<T, O>(
    parser: &mut Parser<T>,
    count: impl RangeBounds<usize>,
    mut f: impl FnMut(&mut Parser<T>) -> Option<O>,
) -> Option<()> {
    let mut succeeded = 0;

    let count_min = match count.start_bound() {
        Bound::Included(n) => *n,
        Bound::Excluded(n) => *n + 1,
        Bound::Unbounded => 0,
    };
    let count_max = match count.end_bound() {
        Bound::Included(n) => *n + 1,
        Bound::Excluded(n) => *n,
        Bound::Unbounded => usize::MAX,
    };

    for _ in 0..count_max {
        match f(parser) {
            Some(_) => succeeded += 1,
            None    => {
                if succeeded < count_min {
                    return None;
                } else {
                    break;
                }
            }
        }
    }

    Some(())
}

#[inline]
pub fn repeat_collect<T, O>(
    parser: &mut Parser<T>,
    count: impl RangeBounds<usize>,
    mut f: impl FnMut(&mut Parser<T>) -> Option<O>,
) -> Option<Vec<O>> {
    let mut result = Vec::new();

    let count_min = match count.start_bound() {
        Bound::Included(n) => *n,
        Bound::Excluded(n) => *n + 1,
        Bound::Unbounded => 0,
    };
    let count_max = match count.end_bound() {
        Bound::Included(n) => *n + 1,
        Bound::Excluded(n) => *n,
        Bound::Unbounded => usize::MAX,
    };

    for _ in 0..count_max {
        match f(parser) {
            Some(o) => result.push(o),
            None    => {
                if result.len() < count_min {
                    return None;
                } else {
                    break;
                }
            }
        }
    }

    Some(result)
}
