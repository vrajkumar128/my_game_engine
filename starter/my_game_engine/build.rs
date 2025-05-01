fn main() {
    let opengl_wrapper_lib_dir = "../opengl_wrapper_lib";
    let opengl_wrapper_lib_src = format!("{}/opengl_wrapper_lib.c", opengl_wrapper_lib_dir);

    let mut build = cc::Build::new();
    build.file(opengl_wrapper_lib_src)
         .include(opengl_wrapper_lib_dir);
    
    // Windows-specific configuration
    if cfg!(target_os = "windows") {
        if let Ok(glfw_path) = std::env::var("GLFW_DIR") {
            build.include(format!("{}/include", glfw_path));
            println!("cargo:rustc-link-search={}/lib-vc2022", glfw_path);
        } else {
            build.include("C:/Program Files/GLFW/include");
            println!("cargo:rustc-link-search=C:/Program Files/GLFW/lib-vc2022");
        }
        
        // Windows library names
        println!("cargo:rustc-link-lib=glfw3");
        println!("cargo:rustc-link-lib=opengl32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=shell32");
    } else { // *nix configuration
        println!("cargo:rustc-link-lib=glfw");
        println!("cargo:rustc-link-lib=GL");
    }
    
    build.compile("openglwrapper");
}