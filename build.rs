use std::path::{Path, PathBuf};

fn main() {
    // 支持通过环境变量 OPENCV_MOBILE_PATH 指定 opencv-mobile 路径
    // 例如: OPENCV_MOBILE_PATH=/lib/opencv-mobile-armv7
    let opencv_path: PathBuf = if let Ok(path) = std::env::var("OPENCV_MOBILE_PATH") {
        Path::new(&path).to_path_buf()
    } else {
        // 默认在当前包目录下查找
        Path::new("opencv-mobile").to_path_buf()
    };

    if !opencv_path.exists() {
        // 当 opencv-mobile 不存在时，跳过构建
        // 用户需要从 https://github.com/nihui/opencv-mobile 下载并放到项目目录
        println!(
            "cargo:warning=opencv-mobile not found at {}. \
            Please download opencv-mobile from https://github.com/nihui/opencv-mobile \
            and extract to the project directory, or set OPENCV_MOBILE_PATH environment variable.",
            opencv_path.display()
        );
        println!("cargo:rerun-if-changed=build.rs");
        return;
    }

    println!("cargo:message=info:Using opencv-mobile path: {}", opencv_path.display());

    let opencv_include = opencv_path.join("include");
    let opencv_include_opencv4 = opencv_path.join("include").join("opencv4");
    let opencv_lib = opencv_path.join("lib");

    // 编译 wrapper.c
    println!("cargo:rerun-if-changed=wrapper.c");
    println!("cargo:rerun-if-changed=build.rs");

    // 使用 cc crate 编译 wrapper.c (作为 C++ 编译)
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| "target".to_string());
    cc::Build::new()
        .cpp(true)
        .file("wrapper.c")
        .include(opencv_include.to_str().unwrap())
        .include(opencv_include_opencv4.to_str().unwrap())
        .flag("-w")
        .out_dir(&target_dir)
        .compile("opencv_wrapper");

    // 链接 OpenCV 静态库
    println!("cargo:rustc-link-search=native={}", opencv_lib.display());

    // 链接 OpenCV 模块
    let use_static = cfg!(feature = "static-link");

    let opencv_libs = [
        "opencv_highgui",
        "opencv_imgproc",
        "opencv_core",
        "opencv_features2d",
        "opencv_photo",
        "opencv_video",
    ];

    for lib in opencv_libs.iter() {
        if use_static {
            println!("cargo:rustc-link-lib=static={}", lib);
        } else {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
    }

    // 系统库
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=gomp");
}
