use std::env;
use std::path::Path;

fn main() {
    let opengl_wrapper_lib_dir = "../opengl_wrapper_lib";
    let opengl_wrapper_lib_src = format!("{}/opengl_wrapper_lib.c", opengl_wrapper_lib_dir);

    cc::Build::new()
        .file(opengl_wrapper_lib_src)
        .include(opengl_wrapper_lib_dir)
        .compile("openglwrapper");

    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=GL");
}