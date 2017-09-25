extern crate bindgen;
use bindgen::builder;

fn main() {
    let bindings = builder().header("include/flex.h")
        .clang_arg("-Wall")
        .clang_arg("-Werror")
        .clang_arg("-O3")
        .clang_arg("-arch x86_64")
        .clang_arg("-arch i386")
        .clang_arg("-std=c11")
        .clang_arg("-Llibs")
        .clang_arg("-lflex")
        .generate()
        .unwrap();

    bindings.write_to_file("src/flex_ll.rs").unwrap();

    println!("cargo:rustc-link-search=libs");
    println!("cargo:rustc-link-lib=flex");
}