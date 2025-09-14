#![feature(sync_unsafe_cell)]

use std::cell::SyncUnsafeCell;

/// A `bool` that is initially always true. It may be set from true to false, but never the other way around. This makes it thread safe since it can only effectively be written to once.
#[derive(Debug)]
pub struct SyncUnsafeOnceCellFalsable {
    inner: SyncUnsafeCell<bool>,
}

impl SyncUnsafeOnceCellFalsable {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: SyncUnsafeCell::new(true),
        }
    }

    /// Returns the `UnsafeCell` containing the `bool`.
    pub const fn get(&self) -> &SyncUnsafeCell<bool> {
        &self.inner
    }

    /// Returns the inner `bool`.
    pub const fn get_bool(self) -> bool {
        self.inner.into_inner()
    }

    /**
    Sets the bool to false.

    ## Safety
    This is thread safe and cannot cause race conditions given the behavior of this type.
    */
    #[allow(invalid_reference_casting)]
    pub const fn set_false(&self) {
        // Dereference raw pointer to inner bool to make it false.
        unsafe { *self.inner.get() = false };
    }
}

impl Default for SyncUnsafeOnceCellFalsable {
    fn default() -> Self {
        Self::new()
    }
}
