use std::{
    mem::MaybeUninit,
    ptr::{self, NonNull},
};

pub(crate) trait Ptr {
    type NonNull;
}

impl<T> Ptr for *mut T {
    type NonNull = NonNull<T>;
}

/// # Safety
///
/// `Self` and `T` must have the same representation in memory.
pub unsafe trait Downcast<T>: Sized {
    fn is_instance_of(&self) -> bool;

    fn downcast(self) -> Result<T, Self> {
        if self.is_instance_of() {
            let this = MaybeUninit::new(self);
            Ok(unsafe { ptr::read(this.as_ptr() as *const T) })
        } else {
            Err(self)
        }
    }

    fn downcast_ref(&self) -> Option<&T> {
        if self.is_instance_of() {
            Some(unsafe { &*(self as *const _ as *const T) })
        } else {
            None
        }
    }

    fn downcast_mut(&mut self) -> Option<&mut T> {
        if self.is_instance_of() {
            Some(unsafe { &mut *(self as *mut _ as *mut T) })
        } else {
            None
        }
    }
}
