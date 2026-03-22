//! C FFI 导出

use crate::runtime::{Runtime, RuntimeConfig};
use std::ffi::CStr;
use std::os::raw::c_char;

/// 创建运行时 (C FFI)
#[no_mangle]
pub extern "C" fn agently_runtime_create(
    agent_id: *const c_char,
    model_path: *const c_char,
) -> *mut Runtime {
    let config = RuntimeConfig {
        agent_id: unsafe { CStr::from_ptr(agent_id).to_string_lossy().into_owned() },
        model_path: unsafe { CStr::from_ptr(model_path).to_string_lossy().into_owned() },
    };
    let runtime = Box::new(Runtime::new(config));
    Box::into_raw(runtime)
}

/// 启动运行时 (C FFI)
#[no_mangle]
pub extern "C" fn agently_runtime_start(runtime: *mut Runtime) -> i32 {
    let runtime = unsafe { &mut *runtime };
    match runtime.start() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
