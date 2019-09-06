use nvml_sys::bindings::nvmlBAR1Memory_t;
use nvml_sys::bindings::nvmlClockType_enum_NVML_CLOCK_MEM;
use nvml_sys::bindings::nvmlClockType_enum_NVML_CLOCK_SM;
use nvml_sys::bindings::nvmlDeviceGetBAR1MemoryInfo;
use nvml_sys::bindings::nvmlDeviceGetClockInfo;
use nvml_sys::bindings::nvmlDeviceGetCudaComputeCapability;
use nvml_sys::bindings::nvmlDeviceGetHandleByIndex_v2;
use nvml_sys::bindings::nvmlDeviceGetMaxPcieLinkGeneration;
use nvml_sys::bindings::nvmlDeviceGetMaxPcieLinkWidth;
use nvml_sys::bindings::nvmlDeviceGetName;
use nvml_sys::bindings::nvmlDeviceGetPciInfo_v3;
use nvml_sys::bindings::nvmlDeviceGetUUID;
use nvml_sys::bindings::nvmlDevice_t;
use nvml_sys::bindings::nvmlPciInfo_t;
use nvml_sys::bindings::nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED;
use nvml_sys::bindings::nvmlReturn_enum_NVML_SUCCESS;
use nvml_sys::bindings::nvmlSystemGetDriverVersion;
use nvml_sys::bindings::NVML_DEVICE_NAME_BUFFER_SIZE;
use nvml_sys::bindings::NVML_DEVICE_UUID_BUFFER_SIZE;
use nvml_sys::bindings::NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE;

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

pub struct PCIInfo {
    pub bus_id: String,
    pub bar1: u64,
    pub bandwidth: u64,
}

pub struct Device {
    pub handler: nvml_sys::bindings::nvmlDevice_t,
    pub name: String,
    pub uuid: String,
    pub model: String,
    pub path: String,
    pub minor_number: i32,
    pub power_management_limit: i32,
    pub memory_info: String,

    pub pci_info: PCIInfo,
    pub bar1_memory_info: String,
    pub max_pcie_link_generation: String,
    pub max_pcie_link_width: String,
    pub max_clock_info: String,
    pub cuda_compute_capability: String,
}

impl Device {
    pub fn new(index: i32) -> Result<Option<Self>, String> {
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
        unsafe {
            let mut pci_info: nvmlPciInfo_t = std::mem::uninitialized();
            // let mut pci_info: std::mem::MaybeUninit<nvmlPciInfo_t> =
            //     std::mem::MaybeUninit::<nvmlPciInfo_t>::uninit();
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

    pub fn get_bar1_memory_info(&self) -> Result<(u64, u64), String> {
        unsafe {
            let mut bar1_memory_info: nvmlBAR1Memory_t = std::mem::uninitialized();
            let result = nvmlDeviceGetBAR1MemoryInfo(
                self.handler,
                &mut bar1_memory_info as *mut nvmlBAR1Memory_t,
            );
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok((
                    bar1_memory_info.bar1Total as u64,
                    bar1_memory_info.bar1Used as u64,
                ));
            }
            Err(result_error(result).unwrap())
        }
    }

    pub fn get_max_pcie_link_generation(&self) -> Result<u64, String> {
        unsafe {
            let mut link: ::std::os::raw::c_uint = 0;
            let result = nvmlDeviceGetMaxPcieLinkGeneration(
                self.handler,
                &mut link as *mut ::std::os::raw::c_uint,
            );
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(link as u64);
            }
            Err(result_error(result).unwrap())
        }
    }

    pub fn get_max_pcie_link_width(&self) -> Result<u64, String> {
        unsafe {
            let mut width: ::std::os::raw::c_uint = 0;
            let result = nvmlDeviceGetMaxPcieLinkWidth(
                self.handler,
                &mut width as *mut ::std::os::raw::c_uint,
            );
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(width as u64);
            }
            Err(result_error(result).unwrap())
        }
    }

    pub fn get_clock_info(&self) -> Result<(u64, u64), String> {
        unsafe {
            let mut sm: ::std::os::raw::c_uint = 0;
            let mut mem: ::std::os::raw::c_uint = 0;
            let mut result = nvmlDeviceGetClockInfo(
                self.handler,
                nvmlClockType_enum_NVML_CLOCK_SM,
                &mut sm as *mut ::std::os::raw::c_uint,
            );
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                result = nvmlDeviceGetClockInfo(
                    self.handler,
                    nvmlClockType_enum_NVML_CLOCK_MEM,
                    &mut mem as *mut ::std::os::raw::c_uint,
                );
            }
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok((sm as u64, mem as u64));
            }
            Err(result_error(result).unwrap())
        }
    }
    pub fn get_cuda_compute_capability(&self) -> Result<(u64, u64), String> {
        unsafe {
            let mut major: ::std::os::raw::c_int = 0;
            let mut minor: ::std::os::raw::c_int = 0;
            let result = nvmlDeviceGetCudaComputeCapability(
                self.handler,
                &mut major as *mut ::std::os::raw::c_int,
                &mut minor as *mut ::std::os::raw::c_int,
            );
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result_error(result).unwrap());
            }
            return Ok((major as u64, minor as u64));
        }
    }
    pub fn numa_node(&self) -> Result<u64, String> {
        Ok(0)
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
