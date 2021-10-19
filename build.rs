use std::path::PathBuf;

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

    const PREPEND_LIB: &'static str = "
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
";

    let out = PathBuf::new().join("src").join("lib.rs");

    let bindings = bindgen::Builder::default().header("src/wrapper.h")
                                              .raw_line(PREPEND_LIB)
                                              .parse_callbacks(Box::new(ParseCallbacks))
                                              .generate_comments(false)
                                              .layout_tests(false)
                                              .ctypes_prefix("libc")
                                              .allowlist_type("[oO]pus.+")
                                              .allowlist_function("[oO]pus.+")
                                              .allowlist_var("[oO].+")
                                              .use_core()
                                              .generate()
                                              .expect("Unable to generate bindings");

    bindings.write_to_file(out).expect("Couldn't write bindings!");

}

#[cfg(not(feature = "build-bindgen"))]
fn generate_lib() {
}

fn get_android_vars() -> Option<(PathBuf, &'static str)> {
    if let Ok(android_ndk) = std::env::var("ANDROID_NDK_HOME") {
        let mut toolchain_file = PathBuf::new();
        toolchain_file.push(android_ndk);
        toolchain_file.push("build");
        toolchain_file.push("cmake");
        toolchain_file.push("android.toolchain.cmake");

        let target = std::env::var("TARGET").unwrap();
        let abi = match target.as_str() {
            "armv7-linux-androideabi" => "armeabi-v7a",
            "aarch64-linux-android" => "arm64-v8a",
            "i686-linux-android" => "x86",
            "x86_64-linux-android" => "x86_64",
            _ => return None,
        };

        Some((toolchain_file, abi))
    } else {
        None
    }
}

fn set_cmake_define_if_present(config: &mut cmake::Config, name: &str) {
    if let Ok(value) = std::env::var(name) {
        config.define(name, value);
    } else if let Ok(value) = std::env::var(format!("CARGO_NDK_{}", name)) {
        config.define(name, value);
    } else {
        println!("cargo:warning=Unable to find Android env variable '{}'. Hope for good default...", name);
    }
}

fn build() {
    const CURRENT_DIR: &'static str = "opus";

    let mut cmake = cmake::Config::new(CURRENT_DIR);
    cmake.define("OPUS_INSTALL_PKG_CONFIG_MODULE", "OFF")
         .define("OPUS_INSTALL_CMAKE_CONFIG_MODULE", "OFF")
         //Defining these variables disable GNUInstallDirs so in addition to /lib
         //define some commonly build stuff too.
         .define("CMAKE_INSTALL_BINDIR", "bin");
         .define("CMAKE_INSTALL_MANDIR", "man");
         .define("CMAKE_INSTALL_INCLUDEDIR", "include");
         .define("CMAKE_INSTALL_OLDINCLUDEDIR", "include");
         .define("CMAKE_INSTALL_LIBDIR", "lib");

    let host = std::env::var("HOST").unwrap();
    let target = std::env::var("TARGET").unwrap();

    if let Some((toolchain_file, abi)) = get_android_vars() {
        cmake.define("CMAKE_TOOLCHAIN_FILE", toolchain_file);
        cmake.define("ANDROID_ABI", abi);

        set_cmake_define_if_present(&mut cmake, "ANDROID_PLATFORM");
        set_cmake_define_if_present(&mut cmake, "ANDROID_STL");
        set_cmake_define_if_present(&mut cmake, "ANDROID_ARM_MODE");
        set_cmake_define_if_present(&mut cmake, "ANDROID_ARM_NEON");

        #[cfg(windows)]
        cmake.generator("Ninja");
    }

    if host == target {
        if !std::is_x86_feature_detected!("avx") {
            cmake.define("OPUS_X86_MAY_HAVE_AVX", "OFF")
                 .define("OPUS_X86_PRESUME_AVX", "OFF")
                 .define("AVX_SUPPORTED", "OFF");
        }

        if !std::is_x86_feature_detected!("sse4.1") {
            cmake.define("OPUS_X86_MAY_HAVE_SSE4_1", "OFF")
                 .define("OPUS_X86_PRESUME_SSE4_1", "OFF")
                 .define("SSE4_1_SUPPORTED", "OFF");
        }

        if !std::is_x86_feature_detected!("sse2") {
            cmake.define("OPUS_X86_MAY_HAVE_SSE2", "OFF")
                 .define("OPUS_X86_PRESUME_SSE2", "OFF")
                 .define("SSE2_SUPPORTED", "OFF");
        }

        if !std::is_x86_feature_detected!("sse") {
            cmake.define("OPUS_X86_MAY_HAVE_SSE", "OFF")
                 .define("OPUS_X86_PRESUME_SSE", "OFF")
                 .define("SSE1_SUPPORTED", "OFF");
        }
    }

    let mut out_dir = cmake.build();

    println!("cargo:rustc-link-lib=static=opus");

    out_dir.push("lib");
    println!("cargo:rustc-link-search=native={}", out_dir.display());
}

fn run() {
    generate_lib();

    build();
}

fn main() {
    run()
}
