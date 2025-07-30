use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new().file("stackarray.c").compile("stackarray");

    let bindings = bindgen::Builder::default()
        .header("stackarray.h")
        .allowlist_function(".*")
        .allowlist_type("Stack")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
