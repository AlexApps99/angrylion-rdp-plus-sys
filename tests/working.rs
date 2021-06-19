#[test]
fn working() {
    unsafe { angrylion_rdp_plus_sys::msg_error(b"Testing\0" as *const u8 as *const i8) }
}
