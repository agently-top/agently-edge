//! C FFI 导出

use crate::runtime::{Runtime, RuntimeConfig};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

/// 全局错误信息缓冲区 (C FFI) - 线程安全
static LAST_ERROR: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn get_last_error() -> &'static Mutex<Option<String>> {
    LAST_ERROR.get_or_init(|| Mutex::new(None))
}

/// 创建运行时 (C FFI)
/// 
/// # Safety
/// 
/// This function is unsafe because it dereferences raw pointers.
/// The caller must ensure that `agent_id` and `model_path` are valid pointers
/// to null-terminated C strings.
#[no_mangle]
pub unsafe extern "C" fn agently_runtime_create(
    agent_id: *const c_char,
    model_path: *const c_char,
) -> *mut Runtime {
    // 验证指针有效性
    if agent_id.is_null() || model_path.is_null() {
        tracing::error!("Null pointer passed to agently_runtime_create");
        return std::ptr::null_mut();
    }

    let config = RuntimeConfig {
        agent_id: CStr::from_ptr(agent_id).to_string_lossy().into_owned(),
        model_path: CStr::from_ptr(model_path).to_string_lossy().into_owned(),
    };
    let runtime = Box::new(Runtime::new(config));
    Box::into_raw(runtime)
}

/// 销毁运行时 (C FFI)
/// 
/// # Safety
/// 
/// This function is unsafe because it dereferences a raw pointer.
/// The caller must ensure that `runtime` is a valid pointer returned by
/// `agently_runtime_create`, and it must not be called twice on the same pointer.
#[no_mangle]
pub unsafe extern "C" fn agently_runtime_destroy(runtime: *mut Runtime) {
    if !runtime.is_null() {
        tracing::debug!("Destroying runtime");
        let _ = Box::from_raw(runtime);
    }
}

/// 获取最后错误信息 (C FFI)
/// 
/// # Safety
/// 
/// This function is unsafe because it returns a raw pointer that must be freed
/// by calling `agently_runtime_free_error`.
#[no_mangle]
pub unsafe extern "C" fn agently_runtime_get_last_error() -> *mut c_char {
    match get_last_error().lock() {
        Ok(guard) => {
            if let Some(ref err) = *guard {
                match CString::new(err.as_str()) {
                    Ok(cstr) => cstr.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            } else {
                std::ptr::null_mut()
            }
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// 释放错误字符串 (C FFI)
/// 
/// # Safety
/// 
/// This function is unsafe because it takes ownership of a raw pointer.
/// The caller must ensure that `ptr` is a valid pointer returned by
/// `agently_runtime_get_last_error`, and it must not be called twice on the same pointer.
#[no_mangle]
pub unsafe extern "C" fn agently_runtime_free_error(ptr: *mut c_char) {
    if !ptr.is_null() {
        tracing::debug!("Freeing error string");
        let _ = CString::from_raw(ptr);
    }
}

/// 启动运行时 (C FFI)
/// 
/// # Safety
/// 
/// This function is unsafe because it dereferences a raw pointer.
/// The caller must ensure that `runtime` is a valid pointer returned by
/// `agently_runtime_create`.
#[no_mangle]
pub unsafe extern "C" fn agently_runtime_start(runtime: *mut Runtime) -> i32 {
    if runtime.is_null() {
        if let Ok(mut guard) = get_last_error().lock() {
            *guard = Some("Null runtime pointer".to_string());
        }
        return -1;
    }

    let runtime = &mut *runtime;
    match runtime.start() {
        Ok(_) => 0,
        Err(e) => {
            if let Ok(mut guard) = get_last_error().lock() {
                *guard = Some(e.to_string());
            }
            -1
        }
    }
}
