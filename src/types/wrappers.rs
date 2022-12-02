use std::{
    cmp::Ordering,
    ffi::{c_char, CStr},
    ops::Deref,
    ptr::copy_nonoverlapping,
    time::{Duration, SystemTime},
};

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EpicsEnum(pub u16);

pub fn time_from_epics(ets: sys::epicsTimeStamp) -> SystemTime {
    SystemTime::UNIX_EPOCH
        + (Duration::from_secs(ets.secPastEpoch as u64) + Duration::from_nanos(ets.nsec as u64))
}

#[derive(Clone, Debug, Eq)]
#[repr(transparent)]
pub struct StaticCString<const N: usize> {
    data: [c_char; N],
}

impl<const N: usize> Default for StaticCString<N> {
    fn default() -> Self {
        Self { data: [0; N] }
    }
}

impl<const N: usize> PartialEq for StaticCString<N> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<const N: usize> PartialOrd for StaticCString<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl<const N: usize> Ord for StaticCString<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.deref().cmp(other.deref())
    }
}

impl<const N: usize> StaticCString<N> {
    pub fn len(&self) -> Option<usize> {
        self.data
            .iter()
            .copied()
            .enumerate()
            .find(|(_, c)| *c == 0)
            .map(|(i, _)| i)
    }
    pub fn is_empty(&self) -> bool {
        self.data[0] == 0
    }
    pub fn from_array(data: [c_char; N]) -> Option<Self> {
        if data.iter().copied().any(|c| c == 0) {
            Some(Self { data })
        } else {
            None
        }
    }
    pub fn from_cstr(cstr: &CStr) -> Option<Self> {
        let bytes = cstr.to_bytes();
        if bytes.len() < N {
            let mut this = Self::default();
            unsafe {
                copy_nonoverlapping(
                    bytes.as_ptr() as *const i8,
                    this.data.as_mut_ptr(),
                    bytes.len() + 1,
                )
            };
            Some(this)
        } else {
            None
        }
    }
}

impl<const N: usize> Deref for StaticCString<N> {
    type Target = CStr;
    fn deref(&self) -> &CStr {
        debug_assert!(
            self.data.iter().copied().any(|c| c == 0),
            "String is not nul-terminated"
        );
        unsafe { CStr::from_ptr(self.data.as_ptr()) }
    }
}

pub type EpicsString = StaticCString<{ sys::MAX_STRING_SIZE as usize }>;
