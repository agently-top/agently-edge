//! Build script for static llama.cpp integration
//! 
//! This script:
//! 1. Clones llama.cpp if not present
//! 2. Compiles llama.cpp as static library
//! 3. Links it to the Rust binary

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let llama_cpp_dir = PathBuf::from("vendor/llama.cpp");
    let build_dir = out_dir.join("llama.cpp/build");

    // 1. 检查 llama.cpp 是否存在，不存在则克隆
    if !llama_cpp_dir.exists() {
        println!("cargo:warning=Cloning llama.cpp...");
        
        fs::create_dir_all(llama_cpp_dir.parent().unwrap()).ok();
        
        let status = Command::new("git")
            .args([
                "clone",
                "--depth", "1",
                "https://github.com/ggerganov/llama.cpp.git",
                llama_cpp_dir.to_str().unwrap(),
            ])
            .status();

        if let Err(e) = status {
            println!("cargo:warning=Failed to clone llama.cpp: {}", e);
            println!("cargo:warning=Using mock mode only");
            println!("cargo:rustc-cfg=feature=\"mock-only\"");
            return;
        }
    }

    println!("cargo:rerun-if-changed=vendor/llama.cpp");

    // 2. 使用 cmake 编译 llama.cpp
    println!("cargo:warning=Building llama.cpp...");
    
    fs::create_dir_all(&build_dir).ok();

    // 配置 cmake
    let mut cmake_cmd = Command::new("cmake");
    cmake_cmd
        .args([
            "-S", llama_cpp_dir.to_str().unwrap(),
            "-B", build_dir.to_str().unwrap(),
            "-DBUILD_SHARED_LIBS=OFF",
            "-DLLAMA_BUILD_TESTS=OFF",
            "-DLLAMA_BUILD_EXAMPLES=OFF",
            "-DLLAMA_BUILD_SERVER=OFF",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DCMAKE_POSITION_INDEPENDENT_CODE=ON",
        ]);

    // 检查是否启用 CUDA
    if env::var("LLAMA_CUDA").unwrap_or_default() == "ON" {
        cmake_cmd.arg("-DGGML_CUDA=ON");
        println!("cargo:warning=CUDA support enabled");
    }

    // 检查是否启用 Metal
    if env::var("LLAMA_METAL").unwrap_or_default() == "ON" {
        cmake_cmd.arg("-DGGML_METAL=ON");
        println!("cargo:warning=Metal support enabled");
    }

    let cmake_status = cmake_cmd.status();

    match cmake_status {
        Ok(status) if status.success() => {
            // 3. 编译
            let build_status = Command::new("cmake")
                .args(["--build", build_dir.to_str().unwrap(), "--target", "llama", "--config", "Release"])
                .status();

            match build_status {
                Ok(status) if status.success() => {
                    // 4. 链接库
                    let lib_path = build_dir.join("src").join("Release");
                    let fallback_lib_path = build_dir.join("src");
                    
                    // 尝试不同的库路径
                    if lib_path.exists() {
                        println!("cargo:rustc-link-search=native={}", lib_path.display());
                    } else if fallback_lib_path.exists() {
                        println!("cargo:rustc-link-search=native={}", fallback_lib_path.display());
                    } else {
                        println!("cargo:warning=Could not find llama library path");
                    }

                    // 链接静态库
                    println!("cargo:rustc-link-lib=static=llama");
                    println!("cargo:rustc-link-lib=static=ggml");
                    println!("cargo:rustc-link-lib=static=ggml-base");
                    
                    // 系统依赖
                    println!("cargo:rustc-link-lib=stdc++");
                    println!("cargo:rustc-link-lib=pthread");
                    
                    // CUDA 依赖
                    if env::var("LLAMA_CUDA").unwrap_or_default() == "ON" {
                        println!("cargo:rustc-link-lib=cudart");
                        println!("cargo:rustc-link-lib=cublas");
                        println!("cargo:rustc-link-lib=cublasLt");
                    }
                    
                    println!("cargo:warning=llama.cpp built and linked successfully");
                }
                Ok(status) => {
                    println!("cargo:warning=Failed to build llama.cpp: exit code {}", status);
                    println!("cargo:warning=Using mock mode");
                    println!("cargo:rustc-cfg=feature=\"mock-only\"");
                }
                Err(e) => {
                    println!("cargo:warning=Failed to build llama.cpp: {}", e);
                    println!("cargo:warning=Using mock mode");
                    println!("cargo:rustc-cfg=feature=\"mock-only\"");
                }
            }
        }
        Ok(status) => {
            println!("cargo:warning=CMake configuration failed: exit code {}", status);
            println!("cargo:warning=Using mock mode");
            println!("cargo:rustc-cfg=feature=\"mock-only\"");
        }
        Err(e) => {
            println!("cargo:warning=CMake not found or failed: {}", e);
            println!("cargo:warning=Using mock mode");
            println!("cargo:rustc-cfg=feature=\"mock-only\"");
        }
    }
}
