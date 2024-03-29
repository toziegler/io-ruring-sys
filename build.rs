use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Inform Cargo when to rerun the build script
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    // Monitor the entire c/liburing directory for changes
    println!("cargo:rerun-if-changed=c/liburing/");

    let liburing_src = Path::new("./c/liburing/src/");
    let liburing_out = Path::new(&out_dir).join("liburing");

    if !Path::new("./c/liburing/src/").exists() {
        let _ = Command::new("git")
            .args(["submodule", "update", "--init", "c/liburing"])
            .status();
    }

    if !liburing_out.exists() {
        Command::new("cp")
            .arg("-r")
            .arg("c/liburing")
            .arg(&out_dir)
            .status()
            .expect("Failed to copy liburing to OUT_DIR");

        Command::new("make")
            .arg("-j")
            .current_dir(liburing_out)
            .status()
            .expect("Failed to build liburing.a");
    }

    println!("cargo:rustc-link-lib=static=uring-ffi");
    println!("cargo:rustc-link-search=native={}/liburing/src", out_dir);

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .allowlist_function("__io_uring.*")
        .allowlist_function("io_uring.*")
        .allowlist_var("IORING.*")
        .allowlist_var("IOSQE.*")
        .allowlist_item("IORING.*")
        .allowlist_item("IOSQE.*")
        .allowlist_type("io_uring.*")
        .clang_arg(format!("-I{}/liburing/src/include", out_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header("c/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    /*let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");


    if !Path::new("./c/liburing/src/").exists() {
        let _ = Command::new("git")
            .args(["submodule", "update", "--init", "c/liburing"])
            .status();
    }

    Command::new("cp")
        .arg("-r")
        .arg("c/liburing")
        .arg(out_dir.clone())
        .status()
        .expect("copy liburing to out_dir");
    Command::new("make")
        .arg("-j")
        .current_dir(format!("{}/liburing", out_dir.clone()))
        .status()
        .expect("failed to build liburing.a");

    //println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=static=uring-ffi");
    println!(
        "cargo:rustc-link-search=native={}/liburing/src",
        out_dir.clone()
    );

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .allowlist_function("__io_uring.*")
        .allowlist_function("io_uring.*")
        .allowlist_var("IORING.*")
        .allowlist_var("IOSQE.*")
        .allowlist_item("IORING.*")
        .allowlist_item("IOSQE.*")
        .allowlist_type("io_uring.*")
        .clang_arg(format!("-I{}/liburing/src/include", out_dir.clone()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header("c/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");*/
}
