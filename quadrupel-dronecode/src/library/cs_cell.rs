use core::cell::UnsafeCell;

/// WARNING: ONLY WORKS ON SINGLE CORE MACHINES. DO NOT USE
/// ON MULTI CORE MACHINES EVER
///
/// Cell-like construct that does not enforce !Send + !Sync
/// since we know that the system is single core. Interrupts
/// are automatically and safely disabled while mutating the
/// contents.
pub struct CSCell<T> {
    cell: UnsafeCell<T>,
}

unsafe impl<T> Sync for CSCell<T> {}

impl<T> CSCell<T>
where
    T: Copy,
{
    pub fn set(&self, v: T) {
        self.update(|i| *i = v);
    }

    /// Set the contents without a critical section. This
    /// may be useful in interrupts when other interrupts can
    /// never happen.
    pub unsafe fn set_unchecked<U>(&self, v: T) {
        self.update_unchecked(|i| *i = v);
    }

    /// Get the contents without a critical section. This
    /// may be useful in interrupts when other interrupts can
    /// never happen.
    pub unsafe fn get_unchecked(&self) -> T {
        self.update_unchecked(|i| *i)
    }

    pub fn get(&self) -> T {
        self.update(|i| *i)
    }
}

impl<T> CSCell<T> {
    pub const fn new(v: T) -> Self {
        Self {
            cell: UnsafeCell::new(v),
        }
    }

    /// Update the contents without a critical section. This
    /// may be useful in interrupts when other interrupts can
    /// never happen.
    /// # Safety: Should only be used in an interrupt
    pub unsafe fn update_unchecked<U>(&self, mut f: impl FnMut(&mut T) -> U) -> U {
        f(&mut *self.cell.get())
    }

    /// Update the contents without a critical section. This
    /// may be useful in interrupts when other interrupts can
    /// never happen.
    /// # Safety: Should only be used in an interrupt
    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.cell.get()
    }

    pub fn update<U>(&self, f: impl FnMut(&mut T) -> U) -> U {
        cortex_m::interrupt::free(|_| unsafe { self.update_unchecked(f) })
    }
}
