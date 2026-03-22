//! C FFI 导出

use crate::runtime::{Runtime, RuntimeConfig};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// 全局错误信息缓冲区 (C FFI)
static mut LAST_ERROR: Option<String> = None;

/// 创建运行时 (C FFI)
#[no_mangle]
pub extern "C" fn agently_runtime_create(
    agent_id: *const c_char,
    model_path: *const c_char,
) -> *mut Runtime {
    // 验证指针有效性
    if agent_id.is_null() || model_path.is_null() {
        tracing::error!("Null pointer passed to agently_runtime_create");
        return std::ptr::null_mut();
    }
    
    let config = RuntimeConfig {
        agent_id: unsafe { CStr::from_ptr(agent_id).to_string_lossy().into_owned() },
        model_path: unsafe { CStr::from_ptr(model_path).to_string_lossy().into_owned() },
    };
    let runtime = Box::new(Runtime::new(config));
    Box::into_raw(runtime)
}

/// 销毁运行时 (C FFI)
#[no_mangle]
pub extern "C" fn agently_runtime_destroy(runtime: *mut Runtime) {
    if !runtime.is_null() {
        tracing::debug!("Destroying runtime");
        unsafe {
            let _ = Box::from_raw(runtime);
        }
    }
}

/// 获取最后错误信息 (C FFI)
#[no_mangle]
pub extern "C" fn agently_runtime_get_last_error() -> *const c_char {
    unsafe {
        match &LAST_ERROR {
            Some(err) => CString::new(err.as_str()).unwrap_or_default().into_raw(),
            None => std::ptr::null(),
        }
    }
}

/// 启动运行时 (C FFI)
#[no_mangle]
pub extern "C" fn agently_runtime_start(runtime: *mut Runtime) -> i32 {
    if runtime.is_null() {
        unsafe {
            LAST_ERROR = Some("Null runtime pointer".to_string());
        }
        return -1;
    }
    
    let runtime = unsafe { &mut *runtime };
    match runtime.start() {
        Ok(_) => 0,
        Err(e) => {
            unsafe {
                LAST_ERROR = Some(e.to_string());
            }
            -1
        }
    }
}
