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
                                              .generate_comments(false)
                                              .generate()
                                              .expect("Unable to generate bindings");

    bindings.write_to_file(out).expect("Couldn't write bindings!");

}

#[cfg(not(feature = "build-bindgen"))]
fn generate_lib() {
}

fn build() {
    const CURRENT_DIR: &'static str = "opus";

    let mut cmake = cmake::Config::new(CURRENT_DIR);
    cmake.define("OPUS_INSTALL_PKG_CONFIG_MODULE", "OFF")
         .define("OPUS_INSTALL_CMAKE_CONFIG_MODULE", "OFF");


    let host = std::env::var("HOST").unwrap();
    let target = std::env::var("TARGET").unwrap();

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
            cmake.define("OPUS_X86_MAY_HAVE_SSE", "OFF")
                 .define("OPUS_X86_PRESUME_SSE", "OFF")
                 .define("SSE2_SUPPORTED", "OFF");
        }

        if !std::is_x86_feature_detected!("sse") {
            cmake.define("OPUS_X86_MAY_HAVE_SSE", "OFF")
                 .define("OPUS_X86_PRESUME_SSE", "OFF")
                 .define("SSE1_SUPPORTED", "OFF");
        }
    }

    let out_dir = cmake.build();

    println!("cargo:rustc-link-lib=static=opus");
    println!("cargo:rustc-link-search=native={}/lib", out_dir.display());
}

fn run() {
    generate_lib();

    build();
}

fn main() {
    run()
}
