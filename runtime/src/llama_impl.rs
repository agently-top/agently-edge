//! llama.cpp FFI 绑定和推理实现

use anyhow::{Context, Result};
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::path::Path;
use std::time::Instant;

use crate::llm::{GenerationResult, GenerateConfig, InferenceStats};

/// llama.cpp 上下文 (opaque pointer)
pub struct LlamaContext {
    ptr: *mut c_void,
}

unsafe impl Send for LlamaContext {}
unsafe impl Sync for LlamaContext {}

impl LlamaContext {
    /// 从 GGUF 文件加载模型并创建上下文
    pub fn load(model_path: &str, n_ctx: u32, n_threads: u32, n_gpu_layers: u32) -> Result<Self> {
        let path_cstr = CString::new(model_path)
            .context("Invalid model path")?;

        // 调用 FFI 加载模型
        let ptr = unsafe {
            llama_cpp_load_model(
                path_cstr.as_ptr(),
                n_ctx as c_int,
                n_threads as c_int,
                n_gpu_layers as c_int,
            )
        };

        if ptr.is_null() {
            anyhow::bail!("Failed to load model: {}", model_path);
        }

        Ok(Self { ptr })
    }

    /// 生成文本
    pub fn generate(&mut self, prompt: &str, config: &GenerateConfig) -> Result<GenerationResult> {
        let prompt_cstr = CString::new(prompt).context("Invalid prompt")?;
        let start = Instant::now();

        // 调用 FFI 生成
        let result_ptr = unsafe {
            llama_cpp_generate(
                self.ptr,
                prompt_cstr.as_ptr(),
                config.max_tokens as c_int,
                config.temperature,
                config.top_p,
            )
        };

        if result_ptr.is_null() {
            anyhow::bail!("Generation failed");
        }

        // 解析结果
        let text = unsafe {
            let c_str = std::ffi::CStr::from_ptr(result_ptr as *const c_char);
            c_str.to_string_lossy().into_owned()
        };

        let duration = start.elapsed();
        let tokens = text.len() as u32 / 4;
        let tokens_per_second = if duration.as_secs_f64() > 0.0 {
            tokens as f64 / duration.as_secs_f64()
        } else {
            0.0
        };

        // 释放结果内存
        unsafe {
            llama_cpp_free_result(result_ptr);
        }

        Ok(GenerationResult {
            text,
            tokens_generated: tokens,
            duration_ms: duration.as_millis() as u64,
            tokens_per_second,
        })
    }

    /// 卸载模型
    pub fn unload(self) {
        unsafe {
            llama_cpp_unload_model(self.ptr);
        }
    }
}

impl Drop for LlamaContext {
    fn drop(&mut self) {
        unsafe {
            llama_cpp_unload_model(self.ptr);
        }
    }
}

// FFI 函数声明
#[link(name = "llama")]
extern "C" {
    fn llama_cpp_load_model(
        path: *const c_char,
        n_ctx: c_int,
        n_threads: c_int,
        n_gpu_layers: c_int,
    ) -> *mut c_void;

    fn llama_cpp_generate(
        ctx: *mut c_void,
        prompt: *const c_char,
        max_tokens: c_int,
        temperature: f32,
        top_p: f32,
    ) -> *mut c_void;

    fn llama_cpp_free_result(result: *mut c_void);

    fn llama_cpp_unload_model(ctx: *mut c_void);
}

/// 使用纯 Rust 实现的简单推理（不依赖 llama.cpp C 库）
pub struct SimpleLlamaEngine {
    model_loaded: bool,
    stats: InferenceStats,
}

impl SimpleLlamaEngine {
    pub fn new() -> Self {
        Self {
            model_loaded: false,
            stats: InferenceStats::default(),
        }
    }

    pub fn load(&mut self, model_path: &str) -> Result<()> {
        let path = Path::new(model_path);
        if !path.exists() {
            anyhow::bail!("Model file not found: {}", model_path);
        }

        // 验证 GGUF 文件头
        let file = std::fs::File::open(path)?;
        let mut reader = std::io::BufReader::new(file);
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        // GGUF magic number: 0x46554747 ("GGUF" in little endian)
        if &magic != b"GGUF" {
            anyhow::bail!("Invalid GGUF file: {}", model_path);
        }

        self.model_loaded = true;
        Ok(())
    }

    pub fn generate(&mut self, prompt: &str, config: &GenerateConfig) -> Result<GenerationResult> {
        if !self.model_loaded {
            anyhow::bail!("Model not loaded");
        }

        // 简化实现：返回 prompt 的回显
        // 实际使用时应替换为真实的 llama.cpp 推理
        let start = Instant::now();
        let text = format!("Generated: {}", prompt);
        let tokens = text.len() as u32 / 4;
        let duration = start.elapsed();

        let tokens_per_second = if duration.as_secs_f64() > 0.0 {
            tokens as f64 / duration.as_secs_f64()
        } else {
            0.0
        };

        self.stats.tokens_per_second = tokens_per_second;
        self.stats.total_tokens_generated += tokens as u64;

        Ok(GenerationResult {
            text,
            tokens_generated: tokens,
            duration_ms: duration.as_millis() as u64,
            tokens_per_second,
        })
    }

    pub fn is_loaded(&self) -> bool {
        self.model_loaded
    }

    pub fn get_stats(&self) -> &InferenceStats {
        &self.stats
    }
}

impl Default for SimpleLlamaEngine {
    fn default() -> Self {
        Self::new()
    }
}

use std::io::Read;
