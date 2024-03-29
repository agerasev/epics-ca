use chrono::{TimeZone, Utc};
use derive_more::{From, Into};
use std::{
    cmp::Ordering,
    ffi::{c_char, CStr},
    fmt::{self, Debug, Formatter},
    ops::Deref,
    ptr::copy_nonoverlapping,
    time::{Duration, SystemTime},
};

#[repr(transparent)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct EpicsEnum(pub u16);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EpicsTimeStamp(pub sys::epicsTimeStamp);

impl EpicsTimeStamp {
    pub fn sec(&self) -> u32 {
        self.0.secPastEpoch
    }
    pub fn nsec(&self) -> u32 {
        self.0.nsec
    }

    pub fn to_system(self) -> SystemTime {
        let unix_epoch = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        let epics_epoch = Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap();
        let diff = (epics_epoch - unix_epoch).to_std().unwrap();
        SystemTime::UNIX_EPOCH
            + diff
            + (Duration::from_secs(self.0.secPastEpoch as u64)
                + Duration::from_nanos(self.0.nsec as u64))
    }
}

impl PartialEq for EpicsTimeStamp {
    fn eq(&self, other: &Self) -> bool {
        self.0.secPastEpoch == other.0.secPastEpoch && self.0.nsec == other.0.nsec
    }
}

impl Eq for EpicsTimeStamp {}

impl PartialOrd for EpicsTimeStamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EpicsTimeStamp {
    fn cmp(&self, other: &Self) -> Ordering {
        let o = self.0.secPastEpoch.cmp(&other.0.secPastEpoch);
        if matches!(o, Ordering::Equal) {
            self.0.nsec.cmp(&other.0.nsec)
        } else {
            o
        }
    }
}

impl Debug for EpicsTimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EpicsTimeStamp {{ sec: {}, nsec: {} }}",
            self.sec(),
            self.nsec()
        )
    }
}

#[derive(Clone, Copy)]
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

impl<const N: usize> Eq for StaticCString<N> {}

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
    pub const MAX_LEN: usize = N - 1;

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
                    bytes.as_ptr() as *const c_char,
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

impl<const N: usize> Debug for StaticCString<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

pub type EpicsString = StaticCString<{ sys::MAX_STRING_SIZE as usize }>;
