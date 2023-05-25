extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/write_to_file.cpp")
        .compile("libwrite_to_file.a");

    let bindings = bindgen::Builder::default()
        .header("src/write_to_file.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
