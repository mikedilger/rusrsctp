
use std::fs;

fn main() {
    let libdir = fs::canonicalize("../usrsctp/usrsctplib/.libs").unwrap();
    println!("cargo:rustc-link-lib=usrsctp");
    println!("cargo:rustc-link-search={}", libdir.to_str().unwrap());
}
