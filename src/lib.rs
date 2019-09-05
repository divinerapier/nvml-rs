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
            match result_error(result) {
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
            Err(result_error(result).unwrap())
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
            Err(result_error(result).unwrap())
        }
    }
}

pub fn result_error(result: nvml_sys::bindings::nvmlReturn_t) -> Option<String> {
    unsafe {
        if result == nvml_sys::bindings::nvmlReturn_enum_NVML_SUCCESS {
            return None;
        }
        let ptr = nvml_sys::bindings::nvmlErrorString(result);
        let message = std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_owned();
        Some(message)
    }
}

pub struct Device {
    pub handler: nvml_sys::bindings::nvmlDevice_t,
    pub name: String,
    pub uuid: String,
    pub minor_number: i32,
    pub power_management_limit: i32,
    pub memory_info: String,

    pub pci_info: String,
    pub bar1_memory_info: String,
    pub max_pcie_link_generation: String,
    pub max_pcie_link_width: String,
    pub max_clock_info: String,
    pub cuda_compute_capability: String,
}

impl Device {
    pub fn new(index: i32) -> Result<Option<Self>, String> {
        use nvml_sys::bindings::nvmlDeviceGetHandleByIndex_v2;
        use nvml_sys::bindings::nvmlDevice_t;
        use nvml_sys::bindings::nvmlReturn_enum_NVML_SUCCESS;
        unsafe {
            let mut dev: nvmlDevice_t = std::ptr::null_mut();
            let result = nvmlDeviceGetHandleByIndex_v2(
                index as ::std::os::raw::c_uint,
                &mut dev as *mut nvmlDevice_t,
            );
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result_error(result).unwrap());
            }
            // get name deviceGetName

            // get uuid deviceGetUUID

            // get minor number deviceGetMinorNumber

            // deviceGetPowerManagementLimit
            // deviceGetMemoryInfo
            // deviceGetPciInfo
            // deviceGetBAR1MemoryInfo
            // deviceGetMaxPcieLinkGeneration
            // deviceGetMaxPcieLinkWidth
            // deviceGetMaxClockInfo
            // deviceGetCudaComputeCapability
            // numaNode

            Ok(None)
        }
    }

    pub fn get_name(&self) -> Result<String, String> {
        use nvml_sys::bindings::nvmlDeviceGetName;
        use nvml_sys::bindings::nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED;
        use nvml_sys::bindings::nvmlReturn_enum_NVML_SUCCESS;
        use nvml_sys::bindings::nvmlSystemGetDriverVersion;
        use nvml_sys::bindings::NVML_DEVICE_NAME_BUFFER_SIZE;
        unsafe {
            let mut name: [::std::os::raw::c_char; NVML_DEVICE_NAME_BUFFER_SIZE as usize] =
                [0; NVML_DEVICE_NAME_BUFFER_SIZE as usize];
            let result =
                nvmlDeviceGetName(self.handler, &mut name[0], NVML_DEVICE_NAME_BUFFER_SIZE);
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(std::ffi::CStr::from_ptr(name.as_ptr() as *const _)
                    .to_str()
                    .unwrap()
                    .to_owned());
            }
            Err(result_error(result).unwrap())
        }
    }
    pub fn get_uuid(&self) -> Result<String, String> {
        use nvml_sys::bindings::nvmlDeviceGetUUID;
        use nvml_sys::bindings::nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED;
        use nvml_sys::bindings::nvmlReturn_enum_NVML_SUCCESS;
        use nvml_sys::bindings::nvmlSystemGetDriverVersion;
        use nvml_sys::bindings::NVML_DEVICE_UUID_BUFFER_SIZE;
        unsafe {
            let mut name: [::std::os::raw::c_char; NVML_DEVICE_UUID_BUFFER_SIZE as usize] =
                [0; NVML_DEVICE_UUID_BUFFER_SIZE as usize];
            let result =
                nvmlDeviceGetUUID(self.handler, &mut name[0], NVML_DEVICE_UUID_BUFFER_SIZE);
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(std::ffi::CStr::from_ptr(name.as_ptr() as *const _)
                    .to_str()
                    .unwrap()
                    .to_owned());
            }
            Err(result_error(result).unwrap())
        }
    }
    pub fn get_pci_info(&self) -> Result<String, String> {
        use nvml_sys::bindings::nvmlDeviceGetPciInfo_v3;
        use nvml_sys::bindings::nvmlPciInfo_t;
        use nvml_sys::bindings::nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED;
        use nvml_sys::bindings::nvmlReturn_enum_NVML_SUCCESS;
        unsafe {
            let mut pci_info: nvmlPciInfo_t = std::mem::uninitialized();
            let result = nvmlDeviceGetPciInfo_v3(self.handler, &mut pci_info as *mut nvmlPciInfo_t);
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(
                    std::ffi::CStr::from_ptr(pci_info.busId.as_ptr() as *const _)
                        .to_str()
                        .unwrap()
                        .to_owned(),
                );
            }
            Err(result_error(result).unwrap())
        }
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
