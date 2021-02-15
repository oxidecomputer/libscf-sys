// Copyright 2021 Oxide Computer Company

use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(not(target_os = "illumos"))]
    compile_error!("libscf-sys is only supported on illumos");

    // Tell cargo to tell rustc to link the system scf shared library.
    println!("cargo:rustc-link-lib=scf");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    if let Err(_) = env::var("LIBCLANG_PATH") {
        // TODO: Would love to have a way to not hardcode this.
        env::set_var("LIBCLANG_PATH", "/opt/ooce/clang-11.0/lib");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
