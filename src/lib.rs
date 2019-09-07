use nvml_sys::bindings::nvmlBAR1Memory_t;
use nvml_sys::bindings::nvmlClockType_enum_NVML_CLOCK_MEM;
use nvml_sys::bindings::nvmlClockType_enum_NVML_CLOCK_SM;
use nvml_sys::bindings::nvmlDeviceGetBAR1MemoryInfo;
use nvml_sys::bindings::nvmlDeviceGetClockInfo;
use nvml_sys::bindings::nvmlDeviceGetCudaComputeCapability;
use nvml_sys::bindings::nvmlDeviceGetHandleByIndex_v2;
use nvml_sys::bindings::nvmlDeviceGetMaxPcieLinkGeneration;
use nvml_sys::bindings::nvmlDeviceGetMaxPcieLinkWidth;
use nvml_sys::bindings::nvmlDeviceGetMemoryInfo;
use nvml_sys::bindings::nvmlDeviceGetMinorNumber;
use nvml_sys::bindings::nvmlDeviceGetName;
use nvml_sys::bindings::nvmlDeviceGetPciInfo_v3;
use nvml_sys::bindings::nvmlDeviceGetPowerManagementLimit;
use nvml_sys::bindings::nvmlDeviceGetUUID;
use nvml_sys::bindings::nvmlDevice_t;
use nvml_sys::bindings::nvmlMemory_t;
use nvml_sys::bindings::nvmlPciInfo_t;
use nvml_sys::bindings::nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED;
use nvml_sys::bindings::nvmlReturn_enum_NVML_SUCCESS;
use nvml_sys::bindings::nvmlReturn_t;
use nvml_sys::bindings::nvmlSystemGetDriverVersion;
use nvml_sys::bindings::NVML_DEVICE_NAME_BUFFER_SIZE;
use nvml_sys::bindings::NVML_DEVICE_UUID_BUFFER_SIZE;
use nvml_sys::bindings::NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE;

// If you want to use the function of c as the callback function of rust,
// the function type must be marked as unsafe extern "C"
type ProcessOneInterger =
    unsafe extern "C" fn(*mut nvml_sys::bindings::nvmlDevice_st, *mut u32) -> u32;

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

impl Drop for NVML {
    fn drop(&mut self) {
        unsafe {
            nvml_sys::bindings::nvmlShutdown_dl();
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

pub struct ClockInfo {
    pub cores: u64,
    pub memory: u64,
}

pub struct P2PLink {
    pub bus_id: String,
    pub link: P2PLinkType,
}

pub enum P2PLinkType {
    P2PLinkUnknown = 0,
    P2PLinkCrossCPU = 1,
    P2PLinkSameCPU = 2,
    P2PLinkHostBridge = 3,
    P2PLinkMultiSwitch = 4,
    P2PLinkSingleSwitch = 5,
    P2PLinkSameBoard = 6,
    SingleNVLINKLink = 7,
    TwoNVLINKLinks = 8,
    ThreeNVLINKLinks = 9,
    FourNVLINKLinks = 10,
    FiveNVLINKLinks = 11,
    SixNVLINKLinks = 12,
}

pub struct CudaComputeCapabilityInfo {
    pub major: u64,
    pub minor: u64,
}

pub struct Device {
    pub handler: Handler,
    pub uuid: String,
    pub path: String,
    pub model: String,
    pub power: Option<u64>,
    pub memory: Option<u64>,
    pub cpu_affinity: Option<u64>,
    pub pci: PCIInfo,
    pub clocks: ClockInfo,
    pub topology: Vec<i8>,
    pub cuda_compute_capability: CudaComputeCapabilityInfo,
}

impl Device {
    pub fn new(index: u32) -> Result<Self, String> {
        let handler = Handler::new(index)?;
        let model = handler.get_name()?;
        let uuid = handler.get_uuid()?;
        let minor_count = handler.get_minor_number()?;
        let power = handler.get_power_management_limit()?;
        let memory_info = handler.get_memory_info()?;
        let bus_id = handler.get_pci_info()?;
        let (bar1, _) = handler.get_bar1_memory_info()?;
        let pcig = handler.get_max_pcie_link_generation()?;
        let pciw = handler.get_max_pcie_link_width()?;
        let (cores, mem) = handler.get_clock_info()?;
        let (major, minor) = handler.get_cuda_compute_capability()?;
        let node = Self::numa_node(&bus_id)?;
        Ok(Device {
            handler,
            uuid,
            path: format!("/dev/nvidia{}", minor_count),
            model,
            power: Some(power),
            memory: Some(memory_info.total),
            cpu_affinity: Some(0),
            pci: PCIInfo {
                bus_id,
                bar1,
                bandwidth: Self::pci_bandwidth(pcig, pciw),
            },
            clocks: ClockInfo { cores, memory: mem },
            topology: vec![],
            cuda_compute_capability: CudaComputeCapabilityInfo { minor, major },
        })
    }

    fn numa_node(bus_id: &str) -> Result<u64, String> {
        let filepath = format!("/sys/bus/pci/devices/{}/numa_node", bus_id.to_lowercase());
        match std::fs::read_to_string(&filepath) {
            Ok(content) => match content.parse() {
                Ok(node) => {
                    if node < 0 {
                        return Ok(0);
                    }
                    return Ok(node);
                }
                Err(e) => return Err(format!("{}", e)),
            },
            Err(e) => return Err(format!("{}", e)),
        };
        Ok(0)
    }
    fn pci_bandwidth(gen: u64, width: u64) -> u64 {
        width
            * match gen {
                1 => 250,
                2 => 500,
                3 => 985,
                4 => 1969,
                _ => 0,
            }
    }
}

pub struct Handler {
    pub dev: nvmlDevice_t,
}

impl Handler {
    pub fn new(index: u32) -> Result<Handler, String> {
        unsafe {
            let mut dev: nvmlDevice_t = std::ptr::null_mut();
            let result = nvmlDeviceGetHandleByIndex_v2(
                index as ::std::os::raw::c_uint,
                &mut dev as *mut nvmlDevice_t,
            );
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result_error(result).unwrap());
            }
            Ok(Handler { dev })
        }
    }
    pub fn get_name(&self) -> Result<String, String> {
        unsafe {
            let mut name: [::std::os::raw::c_char; NVML_DEVICE_NAME_BUFFER_SIZE as usize] =
                [0; NVML_DEVICE_NAME_BUFFER_SIZE as usize];
            let result = nvmlDeviceGetName(self.dev, &mut name[0], NVML_DEVICE_NAME_BUFFER_SIZE);
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
            let result = nvmlDeviceGetUUID(self.dev, &mut name[0], NVML_DEVICE_UUID_BUFFER_SIZE);
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
            let result = nvmlDeviceGetPciInfo_v3(self.dev, &mut pci_info as *mut nvmlPciInfo_t);
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
                self.dev,
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

    pub fn get_clock_info(&self) -> Result<(u64, u64), String> {
        unsafe {
            let mut sm: ::std::os::raw::c_uint = 0;
            let mut mem: ::std::os::raw::c_uint = 0;
            let mut result = nvmlDeviceGetClockInfo(
                self.dev,
                nvmlClockType_enum_NVML_CLOCK_SM,
                &mut sm as *mut ::std::os::raw::c_uint,
            );
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                result = nvmlDeviceGetClockInfo(
                    self.dev,
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
                self.dev,
                &mut major as *mut ::std::os::raw::c_int,
                &mut minor as *mut ::std::os::raw::c_int,
            );
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result_error(result).unwrap());
            }
            return Ok((major as u64, minor as u64));
        }
    }

    pub fn get_memory_info(&self) -> Result<nvmlMemory_t, String> {
        unsafe {
            let mut mem: nvmlMemory_t = std::mem::uninitialized();
            let mut result = nvmlDeviceGetMemoryInfo(self.dev, &mut mem as *mut nvmlMemory_t);
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result_error(result).unwrap());
            }
            Ok(mem)
        }
    }

    pub fn get_minor_number(&self) -> Result<u64, String> {
        unsafe { self.get_one_interger(nvmlDeviceGetMinorNumber) }
    }

    pub fn get_power_management_limit(&self) -> Result<u64, String> {
        unsafe { self.get_one_interger(nvmlDeviceGetPowerManagementLimit) }
    }

    pub fn get_max_pcie_link_generation(&self) -> Result<u64, String> {
        unsafe { self.get_one_interger(nvmlDeviceGetMaxPcieLinkGeneration) }
    }

    pub fn get_max_pcie_link_width(&self) -> Result<u64, String> {
        unsafe { self.get_one_interger(nvmlDeviceGetMaxPcieLinkWidth) }
    }
}

impl Handler {
    extern "C" fn get_one_interger(&self, f: ProcessOneInterger) -> Result<u64, String> {
        unsafe {
            let mut n: ::std::os::raw::c_uint = 0;
            let result = f(self.dev, &mut n as *mut ::std::os::raw::c_uint);
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(String::from("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(n as u64);
            }
            Err(result_error(result).unwrap())
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let nvml = super::NVML::new().unwrap();
    }
}
