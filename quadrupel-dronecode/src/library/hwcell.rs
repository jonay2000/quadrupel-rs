use core::cell::UnsafeCell;
use core::marker::PhantomData;
use crate::library::hwcell::private::InterruptSafety;

pub trait HwCellMainUpdateImpl<T> {
    fn update_impl<U>(&self, f: impl FnOnce(&mut T) -> U) -> U;
}

/// Will be accessed by both main thread + interrupts, needs sync
/// # SAFETY: Only use on single-threaded machines
pub struct HWCell<IntSafety: InterruptSafety, T> {
    cell: UnsafeCell<T>,
    phantom: PhantomData<IntSafety>,
}

impl<IntSafety: InterruptSafety, T> HWCell<IntSafety, T> {
    /// Read the content of the cell.
    /// # SAFETY: This should not be called from an interrupt
    pub fn read_main<U>(&self, f: impl FnOnce(&T) -> U) -> U where Self: HwCellMainUpdateImpl<T> {
        self.update_main(|t| f(t))
    }

    /// Read the content of the cell from the main thread, returns a copy.
    /// # SAFETY: This should not be called from an interrupt
    pub fn get(&self) -> T
        where
            T: Copy,
            Self: HwCellMainUpdateImpl<T>
    {
        self.update_main(|t| *t)
    }

    pub fn ptr(&self) -> *mut T {
        self.cell.get()
    }

    pub fn new(value: T) -> Self {
        Self {
            cell: UnsafeCell::new(value),
            phantom: Default::default(),
        }
    }

    pub fn update_main<U>(&self, f: impl FnOnce(&mut T) -> U) -> U where Self: HwCellMainUpdateImpl<T> {
        self.update_impl(f)
    }
}

impl<T> HwCellMainUpdateImpl<T> for HWCell<SafeWhenInterruptsOff, T> {
    fn update_impl<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        // When accessing from main thread, we need to turn of interrupts to have sync
        cortex_m::interrupt::free(|_| unsafe { f(&mut *self.cell.get()) })
    }
}

impl<T> HWCell<SafeWhenInterruptsOff, T> {
    pub fn update_interrupt<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        unsafe { f(&mut *self.cell.get()) }
    }
}

impl<T> HwCellMainUpdateImpl<T> for HWCell<SafeFromInterrupt, T> {
    fn update_impl<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        unsafe { f(&mut *self.cell.get()) }
    }
}

impl<T> HWCell<SafeFromInterrupt, T> {
    pub fn update_interrupt<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        unsafe { f(&mut *self.cell.get()) }
    }

    pub fn as_ref(&self) -> &T {
        unsafe { &*self.cell.get() }
    }
    pub fn as_mut_ref(&self) -> &mut T {
        unsafe { &mut *self.cell.get() }
    }
}


impl<T> HwCellMainUpdateImpl<T> for HWCell<UnsafeFromInterrupt, T> {
    fn update_impl<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        unsafe { f(&mut *self.cell.get()) }
    }
}

impl<T> HWCell<UnsafeFromInterrupt, T> {
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.cell.get() }
    }
    pub fn as_mut_ref(&self) -> &mut T {
        unsafe { &mut *self.cell.get() }
    }
}

/// private module so no external types can implement InterruptSafety except for the ones below
mod private {
    pub trait InterruptSafety {}
}

/// When a hardware cell is safe to modify from an interrupt and from outside an interrupt
/// (this is when the contained type performs internal synchronization)
pub struct SafeFromInterrupt;

impl private::InterruptSafety for SafeFromInterrupt {}

/// When a hardware cell is safe to modify from an interrupt, but only allowed to be modified
/// outside an interrupt when interrupts are turned off
pub struct SafeWhenInterruptsOff;

impl private::InterruptSafety for SafeWhenInterruptsOff {}

/// Modifying this type of hardware cell from an interrupt is always unsafe.
pub struct UnsafeFromInterrupt;

impl private::InterruptSafety for UnsafeFromInterrupt {}
