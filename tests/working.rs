#[test]
fn working() {
    // Might segfault, as long as it links this probably works
    unsafe { angrylion_rdp_plus_sys::n64video_close() }
}
