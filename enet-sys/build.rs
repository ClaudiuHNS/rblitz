extern crate cc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut build = cc::Build::new();
    if target.contains("windows") {
        build.define("WIN32", Some("1"));
    }
    build
        .warnings(false)
        .include(".")
        .file("enet/callbacks.c")
        .file("enet/host.c")
        .file("enet/list.c")
        .file("enet/packet.c")
        .file("enet/peer.c")
        .file("enet/protocol.c")
        .file("enet/unix.c")
        .file("enet/win32.c")
        .compile("enet");
    println!("cargo:rustc-link-lib=static=enet");
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=winmm");
    }
}
