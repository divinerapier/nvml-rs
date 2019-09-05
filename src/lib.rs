#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct NVML;

impl NVML {
    fn new() -> Result<NVML, String> {
        unsafe {
            let result = nvml_sys::bindings::nvmlInit_dl();
            if result == nvml_sys::bindings::nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND {
                return Err(format!("could not local NVML library"));
            }
            match Self::result_error(result) {
                None => Ok(NVML),
                Some(message) => Err(message),
            }
        }
    }

    fn device_get_count(&self) -> Result<i32, String> {
        unsafe {
            let mut n: ::std::os::raw::c_uint = 0;
            let result = nvml_sys::bindings::nvmlDeviceGetCount_v2(&mut n as *mut u32);
            if result == nvml_sys::bindings::nvmlReturn_enum_NVML_SUCCESS {
                return Ok(n as i32);
            }
            Err(Self::result_error(result).unwrap())
        }
    }

    fn nvml_system_get_driver_version(&self) -> Result<String, String> {
        use nvml_sys::bindings::nvmlReturn_enum_NVML_SUCCESS;
        use nvml_sys::bindings::nvmlSystemGetDriverVersion;
        use nvml_sys::bindings::NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE;
        unsafe {
            let mut driver: [::std::os::raw::c_char;
                NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE as usize] =
                [0; NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE as usize];
            let result =
                nvmlSystemGetDriverVersion(&mut driver[0], NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(std::ffi::CStr::from_ptr(driver.as_ptr() as *const _)
                    .to_str()
                    .unwrap()
                    .to_owned());
            }
            Err(Self::result_error(result).unwrap())
        }
    }

    fn result_error(result: nvml_sys::bindings::nvmlReturn_t) -> Option<String> {
        unsafe {
            if result == nvml_sys::bindings::nvmlReturn_enum_NVML_SUCCESS {
                return None;
            }
            let ptr = nvml_sys::bindings::nvmlErrorString(result);
            let message = std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_owned();
            Some(message)
        }
    }
}

struct Device {}

impl Device {
    pub fn new(index: i32) -> Self {
        unsafe { Device {} }
    }
}

impl Drop for NVML {
    fn drop(&mut self) {
        unsafe {
            nvml_sys::bindings::nvmlShutdown_dl();
        }
    }
}

// fn shutdown() {}
