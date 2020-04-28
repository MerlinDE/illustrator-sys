extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let toml_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sdk_path = toml_path.join("SDK");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .enable_cxx_namespaces()
        .rustified_enum(".*")
        .blacklist_type("char_type")
        //.blacklist_type()
        .clang_arg("-std=c++11")
        .opaque_type("std::*")
        .opaque_type("size_type")
        // and args for include file search path
        .clang_arg(format!(
            "-I{}",
            sdk_path.join("samplecode").join("common").join("includes").display()
        )) // /samplecode/common/**
        .clang_arg(format!(
            "-I{}",
            sdk_path.join("samplecode").join("common").join("includes").join("legacy").display()
        )) // /samplecode/common/**
        .clang_arg(format!(
            "-I{}",
            sdk_path.join("illustratorapi").join("ate").display()
        )) // /illustratorapi/**
        .clang_arg(format!(
            "-I{}",
            sdk_path.join("illustratorapi").join("ate").join("legacy").display()
        )) // /illustratorapi/**
        .clang_arg(format!(
            "-I{}",
            sdk_path.join("illustratorapi").join("ate").join("SloTextdomTypes").display()
        )) // /illustratorapi/**
        .clang_arg(format!(
            "-I{}",
            sdk_path
                .join("illustratorapi")
                .join("illustrator")
                .display()
        )) // /illustratorapi/**
        .clang_arg(format!(
            "-I{}",
            sdk_path
                .join("illustratorapi")
                .join("illustrator")
                .join("actions")
                .display()
        )) // /illustratorapi/**
        .clang_arg(format!(
            "-I{}",
            sdk_path.join("illustratorapi").join("pica_sp").display()
        )) // /illustratorapi/**
       /*.clang_arg(format!(
            "-include-pch {}",
            sdk_path
                .join("samplecode")
                .join("common")
                .join("includes")
                .join("IllustratorSDKDebug.pch")
                .display()
        ))*/ // -include-pch ./SDK/samplecode/common/includes/IllustratorSDKDebug.pch
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreFoundation.framework/Versions/A/Headers/")
        .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Headers/")
        .clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/")
        .clang_arg("-I/Library/Developer/CommandLineTools/usr/include/c++/v1/")
        .clang_arg("-F/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/")
        .clang_arg("-v")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("illustrator-sys.rs"))
        .expect("Couldn't write bindings!");
}
