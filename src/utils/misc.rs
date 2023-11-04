use std::ops::{Bound, Range, RangeBounds};

pub fn unpack_range(rng: impl RangeBounds<usize>) -> Range<usize> {
    let l = match rng.start_bound() {
        Bound::Included(&l) => l,
        Bound::Excluded(&l) => l + 1,
        Bound::Unbounded => panic!("unbounded range start is not supported"),
    };
    let r = match rng.end_bound() {
        Bound::Included(&r) => r + 1,
        Bound::Excluded(&r) => r,
        Bound::Unbounded => panic!("unbounded range end is not supported"),
    };
    l..r
}

pub fn assert_range(rng: &Range<usize>, bounds: Range<usize>, allow_empty: bool) {
    assert!(
        rng.start <= rng.end,
        "{rng:?}: range end should not be less than start"
    );
    assert!(
        allow_empty || rng.start < rng.end,
        "{rng:?}: empty range is not supported"
    );
    assert!(
        bounds.start <= rng.start && rng.end <= bounds.end,
        "{rng:?}: range does not meet {bounds:?} boundary"
    );
}
