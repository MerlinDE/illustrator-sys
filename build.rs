extern crate bindgen;

use bindgen::RustTarget;
use std::env;
use std::path::PathBuf;

use build_helper::*;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=src/wrapper.cpp");

    let sdk_path = get_ai_sdk_path(env!("AISDK_ROOT"));
    let sdk_header_dirs = ["/samplecode/common/**", "/illustratorapi/**"];
    // let sdk_header_dirs = ["/illustratorapi/**"];

    // get SDK include dirs as CLANG options (prefixed with -I)
    let clang_options = get_ai_include_dirs(
        sdk_header_dirs.iter(),
        sdk_path.to_str().unwrap(),
        IncludeDirFormat::CLANG,
    );

    // get SDK include dirs as plain dirs
    let incl_dirs = get_ai_include_dirs(
        sdk_header_dirs.iter(),
        sdk_path.to_str().unwrap(),
        IncludeDirFormat::PLAIN,
    );

    // define options for CLANG
    let clang_args = [
        "-std=c++14",
        // "-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreFoundation.framework/Versions/A/Headers/",
        // "-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Headers/",
        "-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/",
        // "-I/Library/Developer/CommandLineTools/usr/include/c++/v1/",
        "-F/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk/System/Library/Frameworks/",
    ];

    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // We want to add some C/C++ sources files here as a library to link into the overall project
    let c_src = [
        sdk_path
            .clone()
            .join("illustratorapi/illustrator/IAIUnicodeString.cpp"),
        project_root.clone().join("src/wrapper.cpp"),
    ];

    // enable sccache for c compilation
    // if std::process::Command::new("sccache").arg("--version").status().is_ok() {
    //     std::env::set_var("CC", "sccache cc");
    //     std::env::set_var("CXX", "sccache c++");
    // }

    eprintln!("Building adobe_wrappers.");
    let mut c_builder = cc::Build::new();
    let c_build = c_builder
        .cpp(true)
        .define("MAC_ENV", None)
        .files(c_src.iter())
        // .compiler("clang")
        // .flag("-Wno-unused-parameter")
        // .flag("-std=c++14")
        .flag("--verbose");

    for include_dir in incl_dirs {
        c_build.include(include_dir);
    }
    for clang_arg in clang_args.iter() {
        c_build.flag(clang_arg);
    }
    c_build.compile("adobe_wrappers");
    println!("adobe_wrappers done.");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .enable_cxx_namespaces()
        .rustified_enum(".*")
        // .blacklist_type("char_type")
        //.blacklist_type()
        //.blacklist_function("std::*")
        .whitelist_type("AI.*")
        .whitelist_type("ActionParamType")
        .whitelist_type("ai::.*")
        .whitelist_type("AI::.*")
        .whitelist_type("AS.*")
        .whitelist_type("AT.*")
        .whitelist_type("kSP.*")
        .whitelist_type("kAI.*")
        .whitelist_type(".*Err.*")
        .whitelist_type("ai_sys::.*")
        .whitelist_type("k?AI.*Suite.*")
        .whitelist_type("SP.*Suite.*")
        .whitelist_type("SP.*Message.*")
        .whitelist_type("Suites")
        .whitelist_type(".*Plugin.*")
        .whitelist_type("P[A-Z]_InData")
        .whitelist_type("pr::.*")
        .whitelist_type("PiPL::.*")
        .whitelist_function("SP.*Suite.*")
        .whitelist_function("ai::.*")
        .whitelist_function("unicode_string_from_utf8")
        // .whitelist_function(".*Plugin.*")
        // .whitelist_function("Fixup.*")
        .whitelist_function("kSP.*")
        .whitelist_function("kAI.*")
        .whitelist_var("kSP.*")
        .whitelist_var("kAI.*")
        //.whitelist_var("AI*")
        .clang_arg("-std=c++14")
        .opaque_type("std::.*")
        .opaque_type("ai::UnicodeString")
        // .blacklist_item("ai::UnicodeString")
        //.opaque_type("size_type")
        // and args for include file search path
        // .emit_builtins()
        .clang_args(clang_options)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreFoundation.framework/Versions/A/Headers/")
        // .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Headers/")
        .clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/")
        // .clang_arg("-I/Library/Developer/CommandLineTools/usr/include/c++/v1/")
        .clang_arg(
            "-F/Library/Developer/CommandLineTools/SDKs/MacOSX10.15.sdk/System/Library/Frameworks/",
        )
        .clang_arg("-v")
        .rust_target(RustTarget::Nightly)
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
