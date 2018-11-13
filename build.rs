#[cfg(feature = "build-bindgen")]
extern crate bindgen;

#[cfg(feature = "build-bindgen")]
fn generate_lib() {
    #[derive(Debug)]
    struct ParseCallbacks;

    impl bindgen::callbacks::ParseCallbacks for ParseCallbacks {
        fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
            if name.starts_with("OPUS") {
                Some(bindgen::callbacks::IntKind::Int)
            } else {
                None
            }
        }
    }

    use std::path::PathBuf;

    const PREPEND_LIB: &'static str = "
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
";

    let out = PathBuf::new().join("src").join("lib.rs");

    let bindings = bindgen::Builder::default().header("src/wrapper.h")
                                              .raw_line(PREPEND_LIB)
                                              .parse_callbacks(Box::new(ParseCallbacks))
                                              .generate()
                                              .expect("Unable to generate bindings");

    bindings.write_to_file(out).expect("Couldn't write bindings!");

}

#[cfg(not(feature = "build-bindgen"))]
fn generate_lib() {
}

#[cfg(any(unix, target_env="gnu"))]
fn build(out_dir: &std::path::Path) {
    const CURRENT_DIR: &'static str = "opus";
    use std::process::Command;

    let res = Command::new("sh").arg("autogen.sh")
                                .current_dir(CURRENT_DIR)
                                .status()
                                .expect("To execute sh command");

    if !res.success() {
        panic!("Failed to autogen libopus");
    }

    let res = Command::new("sh").arg("configure")
                                .arg("--disable-shared")
                                .arg("--enable-static")
                                .arg("--disable-doc")
                                .arg("--disable-extra-programs")
                                .arg("--with-pic")
                                .arg("--prefix")
                                .arg(out_dir.to_str().expect("To unwrap out_dir").replace("\\", "/"))
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

    println!("cargo:rustc-link-lib=static=opus");
    println!("cargo:rustc-link-search=native={}/lib", out_dir.display());
}

#[cfg(all(windows, target_env="msvc"))]
fn build(_: &std::path::Path) {
    #[cfg(target_arch = "x86")]
    const LIB_DIR: &'static str = "x86";
    #[cfg(target_arch = "x86_64")]
    const LIB_DIR: &'static str = "x64";

    let lib_dir = std::path::Path::new("prebuilt").join("msvc").join(LIB_DIR).canonicalize().expect("canonicalize");

    //on MSVC we need full name of lib
    println!("cargo:rustc-link-lib=static=libopus");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
}

fn run() {
    generate_lib();

    let out_dir = std::env::var("OUT_DIR").expect("To have OUT_DIR in build script");
    let out_path = std::path::Path::new(&out_dir);

    build(&out_path);
}

fn main() {
    run()
}
