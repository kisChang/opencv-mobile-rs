use std::fs;
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

    println!("cargo:warning=Using opencv-mobile path: {}", opencv_path.display());

    let opencv_include = opencv_path.join("include");
    let opencv_include_opencv4 = opencv_path.join("include").join("opencv4");

    // 自动检测 OpenCV 库目录和版本后缀
    let (opencv_lib, suffix) = match detect_opencv_lib_and_suffix(&opencv_path) {
        Some((lib_dir, suf)) => (lib_dir, suf),
        None => {
            println!("cargo:warning=Cannot find OpenCV library in any expected directory");
            println!("cargo:rerun-if-changed=build.rs");
            return;
        }
    };

    if !suffix.is_empty() {
        println!("cargo:warning=OpenCV library version: {}", suffix);
    }

    // 编译 wrapper.cpp
    println!("cargo:rerun-if-changed=wrapper.cpp");
    println!("cargo:rerun-if-changed=build.rs");

    // 使用 cc crate 编译 wrapper.cpp (作为 C++ 编译)
    let target_dir = std::path::PathBuf::from(
        std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string())
    );
    cc::Build::new()
        .cpp(true)
        .file("wrapper.cpp")
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
            println!("cargo:rustc-link-lib=static={}{}", lib, suffix);
        } else {
            println!("cargo:rustc-link-lib=dylib={}{}", lib, suffix);
        }
    }

    // 系统库
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=msvcrt");
        // Windows GDI 和 UI 库 (highgui 需要)
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=comdlg32");
        println!("cargo:rustc-link-lib=advapi32");
        // COM 库 (opencv_core 需要)
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=uuid");
        // Shell 支持
        println!("cargo:rustc-link-lib=shell32");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=gomp");
    }
}

/// 自动检测 OpenCV 库目录和版本后缀
/// 返回 (库目录路径, 版本后缀)
fn detect_opencv_lib_and_suffix(opencv_path: &Path) -> Option<(PathBuf, String)> {
    // 递归遍历所有子目录，查找包含 opencv_core 库的目录
    search_recursive(opencv_path)
}

/// 递归搜索目录查找 OpenCV 库
fn search_recursive(dir: &Path) -> Option<(PathBuf, String)> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => {
            return None;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // 如果是目录，递归搜索
        if path.is_dir() {
            if let Some(result) = search_recursive(&path) {
                return Some(result);
            }
            continue;
        }

        // 检查文件
        let filename = entry.file_name();
        let filename_str = filename.to_string_lossy();

        #[cfg(target_os = "windows")]
        {
            if filename_str.starts_with("opencv_core") && filename_str.ends_with(".lib") {
                let suffix = filename_str
                    .trim_start_matches("opencv_core")
                    .trim_end_matches(".lib")
                    .to_string();
                return Some((dir.to_path_buf(), suffix));
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            if filename_str == "libopencv_core.a" || filename_str == "libopencv_core.so" || filename_str == "libopencv_core.dylib" {
                return Some((dir.to_path_buf(), String::new()));
            }
            if filename_str.starts_with("libopencv_core.") {
                let prefix = "libopencv_core.";
                if let Some(s) = filename_str.strip_prefix(prefix) {
                    // 移除末尾的 .a 或 .so/.dylib 后缀获取版本号
                    let suffix = if s.ends_with(".a") {
                        s.strip_suffix(".a").unwrap_or(s).to_string()
                    } else if s.ends_with(".so") {
                        s.strip_suffix(".so").unwrap_or(s).to_string()
                    } else if s.contains(".so.") {
                        // 处理 libopencv_core.so.4.5 格式
                        if let Some(pos) = s.find(".so.") {
                            s[pos + 4..].to_string()
                        } else {
                            s.to_string()
                        }
                    } else if s.ends_with(".dylib") {
                        s.strip_suffix(".dylib").unwrap_or(s).to_string()
                    } else {
                        s.to_string()
                    };
                    return Some((dir.to_path_buf(), suffix));
                }
            }
        }
    }

    None
}