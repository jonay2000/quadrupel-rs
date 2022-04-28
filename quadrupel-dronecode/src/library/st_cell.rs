use core::cell::UnsafeCell;

/// WARNING: ONLY WORKS ON SINGLE CORE MACHINES. DO NOT USE
/// ON MULTI CORE MACHINES EVER
///
/// Cell-like construct that does not enforce !Send + !Sync
/// since we know that the system is single core.
/// The contents of this cell will not be used in interrupts, so having mut access to it is safe.
pub struct STCell<T> {
    cell: UnsafeCell<T>,
}

impl<T> STCell<T> {
    pub const fn new(v: T) -> Self {
        Self {
            cell: UnsafeCell::new(v),
        }
    }
}