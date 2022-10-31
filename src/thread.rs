#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct epicsThreadOSD {
    _unused: [u8; 0],
}

pub type epicsThreadId = *mut epicsThreadOSD;
