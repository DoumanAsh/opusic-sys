#[cfg(feature = "build-bindgen")]
extern crate bindgen;

#[cfg(feature = "build-bindgen")]
fn generate_lib() {
    use std::path::PathBuf;

    const PREPEND_LIB: &'static str = "
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
";

    let out = PathBuf::new().join("src").join("lib.rs");

    let bindings = bindgen::Builder::default().header("src/wrapper.h")
                                              .raw_line(PREPEND_LIB)
                                              .generate()
                                              .expect("Unable to generate bindings");

    bindings.write_to_file(out).expect("Couldn't write bindings!");

}

#[cfg(feature = "build-bindgen")]
fn run() {
    generate_lib();
}

#[cfg(not(feature = "build-bindgen"))]
#[cfg(any(unix, target_env="gnu"))]
fn build(out_dir: &std::path::Path) {
    const CURRENT_DIR: &'static str = "opus";
    use std::process::Command;

    let res = Command::new("sh").arg("configure")
                                .arg("--disable-shared")
                                .arg("--enable-static")
                                .arg("--disable-doc")
                                .arg("--disable-extra-programs")
                                .arg("--with-pic")
                                .arg("--prefix")
                                .arg(out_dir.to_str().expect("To unwrap out_dir"))
                                .current_dir(CURRENT_DIR)
                                .status()
                                .expect("To execute sh command");

    if !res.success() {
        panic!("Failed to configure libopus");
    }

    let res = Command::new("make").current_dir(CURRENT_DIR)
                                  .status()
                                  .expect("To execute sh command");

    if !res.success() {
        panic!("Failed to build libopus");
    }

    let res = Command::new("make").current_dir(CURRENT_DIR)
                                  .arg("install")
                                  .status()
                                  .expect("To execute sh command");

    if !res.success() {
        panic!("Failed to install libopus");
    }

    println!("cargo:rustc-flags=-L native={}/lib -l static=opus", out_dir);
}

#[cfg(not(feature = "build-bindgen"))]
#[cfg(all(windows, target_env="msvc"))]
fn build(_: &std::path::Path) {
    #[cfg(target_arch = "x86")]
    const LIB_DIR: &'static str = "x86";
    #[cfg(target_arch = "x86_64")]
    const LIB_DIR: &'static str = "x64";

    let lib_dir = std::path::Path::new("prebuilt").join("msvc").join(LIB_DIR).canonicalize().expect("canonicalize");

    println!("cargo:rustc-flags=-L native={} -l static=opus", lib_dir.display());
}

#[cfg(not(feature = "build-bindgen"))]
fn run() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = std::path::Path::new(&out_dir);

    build(&out_path);
}

fn main() {
    run()
}
