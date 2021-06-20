// Hardcoded for Linux, submit a PR to get this working on Windows :)
const LIB: &str = "mupen64plus-video-angrylion-plus";
const BUILD_PATH: &str = "build/";
const LIB_PATH: &str = "build/mupen64plus-video-angrylion-plus.so";
const NEW_LIB_PATH: &str = "build/libmupen64plus-video-angrylion-plus.so";

#[cfg(not(feature = "no-build"))]
fn main() {
    let p = cmake::Config::new("angrylion-rdp-plus/")
        .profile("Release")
        .build_target(LIB)
        .build();
    std::fs::copy(p.join(LIB_PATH), p.join(NEW_LIB_PATH)).expect("Could not copy library");
    println!(
        "cargo:rustc-link-search=native={}",
        p.join(BUILD_PATH).display()
    );
    println!("cargo:rustc-link-lib=dylib={}", LIB);
    //let b = bindgen::builder()
    //    .header("headers.h")
    //    .generate_comments(false)
    //    .layout_tests(false)
    //    .blocklist_function("dl.*")
    //    .clang_args(&[
    //        "-Iangrylion-rdp-plus/src/",
    //        "-DM64P_CORE_PROTOTYPES",
    //        "-DM64P_PLUGIN_PROTOTYPES",
    //    ])
    //    .generate()
    //    .expect("Could not generate bindings");
    //b.write_to_file("src/bindings.rs")
    //    .expect("Could not save bindings");
}

#[cfg(feature = "no-build")]
fn main() {}
