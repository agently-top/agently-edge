//! Build script for llama.cpp integration

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // 检查是否设置了 LLAMA_CPP_PATH 环境变量
    let llama_cpp_path = match env::var("LLAMA_CPP_PATH") {
        Ok(path) => path,
        Err(_) => {
            // 默认尝试在 vendor 目录查找
            let vendor_path = "vendor/llama.cpp".to_string();
            if std::path::Path::new(&vendor_path).exists() {
                vendor_path
            } else {
                // 如果没有，使用 mock 模式
                println!("cargo:warning=llama.cpp not found, using mock mode");
                println!("cargo:rustc-cfg=feature=\"mock-only\"");
                return;
            }
        }
    };

    println!("cargo:rerun-if-env-changed=LLAMA_CPP_PATH");
    println!("cargo:root={}", llama_cpp_path);

    // 编译 llama.cpp
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("llama.cpp");

    // 如果 vendor 目录存在，复制源码
    if std::path::Path::new(&llama_cpp_path).exists() {
        // 使用 cmake 编译
        let status = Command::new("cmake")
            .args([
                "-S", &llama_cpp_path,
                "-B", build_dir.to_str().unwrap(),
                "-DBUILD_SHARED_LIBS=OFF",
                "-DLLAMA_BUILD_TESTS=OFF",
                "-DLLAMA_BUILD_EXAMPLES=OFF",
            ])
            .status();

        if let Ok(status) = status {
            if status.success() {
                // 编译
                let build_status = Command::new("cmake")
                    .args(["--build", build_dir.to_str().unwrap(), "--target", "llama"])
                    .status();

                if let Ok(build_status) = build_status {
                    if build_status.success() {
                        // 链接库
                        let lib_path = build_dir.join("src");
                        println!("cargo:rustc-link-search=native={}", lib_path.display());
                        println!("cargo:rustc-link-lib=static=llama");
                        println!("cargo:rustc-link-lib=static=ggml");
                        
                        // 需要链接的依赖
                        println!("cargo:rustc-link-lib=stdc++");
                        
                        println!("cargo:warning=llama.cpp compiled successfully");
                        return;
                    }
                }
            }
        }
    }

    // 如果编译失败，使用 mock 模式
    println!("cargo:warning=Failed to compile llama.cpp, using mock mode");
    println!("cargo:rustc-cfg=feature=\"mock-only\"");
}
