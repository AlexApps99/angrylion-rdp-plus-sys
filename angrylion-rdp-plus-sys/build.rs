#[cfg(feature = "build")]
fn main() {
    let mut obj_loc = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    obj_loc.push("angrylion-rdp-plus/src/core/parallel.o");
    cc::Build::new()
        .cpp(true)
        .cargo_metadata(false)
        .debug(false)
        .opt_level(3)
        .warnings(false)
        .include("angrylion-rdp-plus/src/")
        .file("angrylion-rdp-plus/src/core/parallel.cpp")
        // There's no way to only compile an object file
        // So this is an ugly workaround
        // Oh well...
        .compile("useless_library_do_not_touch");
    // Two compilations are needed so C++'s mangling does not break things
    cc::Build::new()
        .debug(false)
        .opt_level(3)
        .warnings(false)
        .object(&obj_loc)
        .include("angrylion-rdp-plus/src/")
        .files(&[
            "angrylion-rdp-plus/src/core/n64video.c",
            "angrylion-rdp-plus/src/core/n64video/vi.c",
            "angrylion-rdp-plus/src/core/n64video/vi/divot.c",
            "angrylion-rdp-plus/src/core/n64video/vi/fetch.c",
            "angrylion-rdp-plus/src/core/n64video/vi/gamma.c",
            "angrylion-rdp-plus/src/core/n64video/vi/lerp.c",
            "angrylion-rdp-plus/src/core/n64video/vi/restore.c",
            "angrylion-rdp-plus/src/core/n64video/vi/video.c",
            "angrylion-rdp-plus/src/core/n64video/rdp.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/blender.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/combiner.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/coverage.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/dither.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/fbuffer.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/rasterizer.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/rdram.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/tcoord.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/tex.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/tmem.c",
            "angrylion-rdp-plus/src/core/n64video/rdp/zbuffer.c",
        ])
        .compile("alp-core");
}

#[cfg(not(feature = "build"))]
fn main() {}
