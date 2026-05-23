use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("target os");
    if target_os != "linux" {
        panic!("hdhomerun-backend Unit 2 currently supports Linux builds only");
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let lib_dir = env::var("HDHR_LIBHDHOMERUN_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.join("../../libhdhomerun"));

    if !lib_dir.join("hdhomerun_channels.c").is_file() {
        panic!(
            "libhdhomerun sources were not found at {}. Set HDHR_LIBHDHOMERUN_DIR or provide the sibling libhdhomerun checkout.",
            lib_dir.display()
        );
    }

    println!("cargo:rerun-if-changed={}", lib_dir.display());
    println!("cargo:rerun-if-env-changed=HDHR_LIBHDHOMERUN_DIR");

    let mut build = cc::Build::new();
    build
        .include(&lib_dir)
        .define("DLL_EXPORT", None)
        .warnings(false)
        .flag_if_supported("-fPIC");

    for file in [
        "hdhomerun_channels.c",
        "hdhomerun_channelscan.c",
        "hdhomerun_control.c",
        "hdhomerun_debug.c",
        "hdhomerun_device.c",
        "hdhomerun_device_selector.c",
        "hdhomerun_discover.c",
        "hdhomerun_os_posix.c",
        "hdhomerun_pkt.c",
        "hdhomerun_sock.c",
        "hdhomerun_sock_posix.c",
        "hdhomerun_sock_netlink.c",
        "hdhomerun_video.c",
    ] {
        build.file(lib_dir.join(file));
    }

    build.compile("hdhomerun_vendor");

    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=rt");
}