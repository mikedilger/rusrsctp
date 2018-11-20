extern crate bindgen;

use std::fs;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Build usrsctp C library
    if ! Path::new("../usrsctp/configure").exists() {
        let bin = fs::canonicalize("../usrsctp/bootstrap").unwrap();
        Command::new(bin)
            .current_dir("../usrsctp")
            .status()
            .expect("Failed to run bootstrap in ../usrsctp");
    }
    if ! Path::new("../usrsctp/Makefile").exists() {
        let bin = fs::canonicalize("../usrsctp/configure").unwrap();
        Command::new(bin)
            .current_dir("../usrsctp")
            .status()
            .expect("Failed to run configure in ../usrsctp");
    }
    Command::new("make")
        .current_dir("../usrsctp")
        .status()
        .expect("Failed to run make in ../usrsctp");

    // Tell cargo to tell rustc to link the library.
    println!("cargo:rustc-link-lib=usrsctp");
    println!("cargo:rustc-link-search=../usrsctp/usrsctplib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
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
