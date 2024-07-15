#![allow(unused_imports)]
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=dylib=haskell");
    println!("cargo:rustc-link-lib=dylib=lincoln");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("libhaskell.so");
    std::fs::copy("libhaskell.so", dest_path).unwrap();
}
