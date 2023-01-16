use std::num::NonZeroU64;

/// Returns `size` as a multiple of `alignment`. See [align].
pub fn align_nonzero(alignment: NonZeroU64, size: NonZeroU64) -> NonZeroU64 {
    let alignment = alignment.get();
    let size = size.get();

    // TODO: Check if this will ever be zero, and if so, in which circunstances
    NonZeroU64::new((size + alignment - 1) & !(alignment - 1)).unwrap()
}

/// Returns `size` as a multiple of `alignment`. Same as `u64::next_multiple_of`.
// TODO: Replace this with `next_multiple_of` once it is stabilized.
// https://github.com/rust-lang/rust/issues/88581
pub fn align(alignment: u64, size: u64) -> u64 {
    (size + alignment - 1) & !(alignment - 1)
}
