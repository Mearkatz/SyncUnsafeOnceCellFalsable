/// A `bool` that is initially always true. It may be set from true to false, but never the other way around. This makes it thread safe since it can only effectively be written to once.
#[derive(Debug, Copy, Clone)]
pub struct SyncUnsafeOnceCellFalsable {
    inner: bool,
}

impl SyncUnsafeOnceCellFalsable {
    #[must_use]
    pub const fn new() -> Self {
        Self { inner: true }
    }

    /// Returns a copy of the inner bool.
    #[must_use]
    pub const fn get(&self) -> bool {
        self.inner
    }

    /// Sets the bool to false.
    #[allow(invalid_reference_casting, clippy::missing_safety_doc)]
    pub const fn set_false(&self) {
        // // Dereference raw pointer to inner bool to make it false.
        // unsafe { *self.inner.get() = false };

        let raw_ptr: *mut Self = std::ptr::from_ref::<Self>(self).cast_mut();
        let mut_ref: &mut Self = unsafe { &mut *raw_ptr };
        mut_ref.inner = false;

        // The above but in one line
        // unsafe { &mut *std::ptr::from_ref::<Self>(self).cast_mut() }.inner = false;
    }
}

impl Default for SyncUnsafeOnceCellFalsable {
    fn default() -> Self {
        Self::new()
    }
}
