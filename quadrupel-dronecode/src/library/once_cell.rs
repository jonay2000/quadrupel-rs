use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct OnceCell<T> {
    is_set: AtomicBool,
    value: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T> Sync for OnceCell<T> {}

impl<T> OnceCell<T> {
    pub const fn new() -> Self {
        Self {
            is_set: AtomicBool::new(false),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn new_with(v: T) -> Self {
        Self {
            is_set: AtomicBool::new(true),
            value: UnsafeCell::new(MaybeUninit::new(v)),
        }
    }

    /// panics on second invocation.
    pub fn initialize(&self, v: T) {
        if self.is_set.load(Ordering::SeqCst) {
            panic!("contents already initialized");
        } else {
            unsafe { *self.value.get() = MaybeUninit::new(v) };

            self.is_set.store(true, Ordering::SeqCst);
        }
    }
}

impl<T> Deref for OnceCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if self.is_set.load(Ordering::SeqCst) {
            unsafe { (&*self.value.get()).assume_init_ref() }
        } else {
            panic!("contents have not yet been initialized");
        }
    }
}
