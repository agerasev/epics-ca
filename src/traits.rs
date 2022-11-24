use std::ptr::NonNull;

pub(crate) trait Ptr {
    type NonNull;
}

impl<T> Ptr for *mut T {
    type NonNull = NonNull<T>;
}

pub trait Downcast<T>: Sized {
    fn is_instance_of(&self) -> bool;

    /// # Safety
    ///
    /// [`Self::is_instance_of`] must be `true`.
    fn downcast_unchecked(self) -> T;

    fn downcast(self) -> Result<T, Self> {
        if self.is_instance_of() {
            Ok(self.downcast_unchecked())
        } else {
            Err(self)
        }
    }
}
