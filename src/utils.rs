use std::ptr::NonNull;

pub(crate) trait Ptr {
    type NonNull;
}

impl<T> Ptr for *mut T {
    type NonNull = NonNull<T>;
}
