use crate as sys;

extern "C" {
    fn test_set_ca_access_rights(
        none: *mut sys::caar,
        read: *mut sys::caar,
        write: *mut sys::caar,
        all: *mut sys::caar,
    );
}

#[test]
fn ca_access_rights() {
    let mut values = [0 as sys::caar; 4];
    unsafe {
        test_set_ca_access_rights(
            &mut values[0] as *mut _,
            &mut values[1] as *mut _,
            &mut values[2] as *mut _,
            &mut values[3] as *mut _,
        );
    }
    assert_eq!(values[0], 0);
    assert_eq!(values[1], sys::CA_READ_ACCESS);
    assert_eq!(values[2], sys::CA_WRITE_ACCESS);
    assert_eq!(values[3], sys::CA_READ_ACCESS | sys::CA_WRITE_ACCESS);
}
