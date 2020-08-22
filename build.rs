extern crate bindgen;

use bindgen::RustTarget;
use glob::glob_with;
use glob::MatchOptions;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let sdk_path: PathBuf = env!("AISDK_ROOT")
        .to_string()
        .parse()
        .expect("AISDK_ROOT env variable configuration error.");

    if !sdk_path.exists() {
        eprintln!(
            "Please download & unpack the Illustrator SDK into {}",
            sdk_path.display()
        );
        std::process::exit(1);
    }

    let sdk_header_dirs = ["/samplecode/common/**", "/illustratorapi/**"];
    // let sdk_header_dirs = ["/illustratorapi/**"];

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut clang_options = Vec::new();
    let mut incl_dirs = Vec::new();

    for hdir in sdk_header_dirs.iter() {
        for entry in glob_with(
            &format!("{}{}", sdk_path.display(), *hdir).to_string(),
            options,
        )
        .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    let ipath = sdk_path.join(path);
                    clang_options.push(format!("-I{}", &ipath.display()));
                    incl_dirs.push(format!("{}", ipath.display()));
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    // We want to add some C/C++ sources files here as a library to link into the overall project
    let c_src = [
        sdk_path
            .clone()
            .join("illustratorapi/illustrator/IAIUnicodeString.cpp"),
        // sdk_path
        //     .clone()
        //     .join("illustratorapi/illustrator/IAIUnicodeString.inl"),
    ];
    let mut c_builder = cc::Build::new();
    let c_build = c_builder
        .cpp(true)
        .define("MAC_ENV", None)
        .files(c_src.iter())
        .flag("-Wno-unused-parameter")
        .flag("-std=c++14")
        .flag("--verbose");

    // .define("USE_ZLIB", None);
    for include_dir in incl_dirs {
        c_build.include(include_dir);
    }
    c_build.compile("adobe_wrappers");

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
        // .whitelist_function(".*Plugin.*")
        // .whitelist_function("Fixup.*")
        .whitelist_function("kSP.*")
        .whitelist_function("kAI.*")
        .whitelist_var("kSP.*")
        .whitelist_var("kAI.*")
        //.whitelist_var("AI*")
        .clang_arg("-std=c++14")
        .opaque_type("std::.*")
        // .opaque_type("ai::UnicodeString")
        // .blacklist_item("ai::UnicodeString")
        //.opaque_type("size_type")
        // and args for include file search path
        // .emit_builtins()
        .clang_args(clang_options)

        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreFoundation.framework/Versions/A/Headers/")
        .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Headers/")
        .clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/")
        .clang_arg("-I/Library/Developer/CommandLineTools/usr/include/c++/v1/")
        .clang_arg("-F/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/")
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
