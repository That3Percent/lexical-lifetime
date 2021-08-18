use std::marker::PhantomData;

/// Extends a lifetime to the end of the lexical scope.
pub struct LexicalLifetime<'a> {
    _marker: PhantomData<&'a ()>,
}

impl LexicalLifetime<'_> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl Drop for LexicalLifetime<'_> {
    #[inline(always)]
    // This line is where the magic happens. The drop impl forces
    // the compiler to extend the lifetime to the end of the lexical
    // scope where the value is dropped. Without this, the lifetime is
    // invalidated as soon as the owning value is no longer used.
    fn drop(&mut self) {}
}
