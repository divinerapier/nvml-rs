#[link(name = "nvidia-ml")]
use ::std::os::raw::{c_char, c_int, c_uchar, c_uint};
use nvml_binding::*;

pub mod error;
pub mod unit;

use error::{Error, Result};

// If you want to use the function of c as the callback function of rust,
// the function type must be marked as unsafe extern "C"
type ProcessOneInterger = unsafe extern "C" fn(*mut nvmlDevice_st, *mut u32) -> u32;

pub struct NVML;

impl NVML {
    pub fn new() -> Result<NVML> {
        unsafe {
            let result = nvmlInit_v2();
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(NVML);
            }
            Err(result.into())
        }
    }

    pub fn device_count(&self) -> Result<u32> {
        unsafe {
            let mut n: ::std::os::raw::c_uint = 0;
            let result = nvmlDeviceGetCount_v2(&mut n as *mut u32);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(n as u32);
            }
            Err(result.into())
        }
    }

    pub fn driver_version(&self) -> Result<String> {
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
            Err(result.into())
        }
    }

    pub fn version(&self) -> Result<String> {
        unsafe {
            let mut driver: [::std::os::raw::c_char;
                NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE as usize] =
                [0; NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE as usize];
            let result =
                nvmlSystemGetDriverVersion(&mut driver[0], NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(std::ffi::CStr::from_ptr(driver.as_ptr() as *const _)
                    .to_str()
                    .unwrap()
                    .to_owned());
            }
            Err(result.into())
        }
    }

    pub fn cuda_version(&self) -> Result<u64> {
        unsafe {
            let mut version = 0;
            let result = nvmlSystemGetCudaDriverVersion(&mut version as *mut ::std::os::raw::c_int);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(version as u64);
            }
            Err(result.into())
        }
    }
    pub fn unit_count(&self) -> Result<u64> {
        unsafe {
            let mut unit_count: c_uint = 0;
            let result = nvmlUnitGetCount(&mut unit_count as *mut c_uint);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(unit_count as u64);
            }
            Err(result.into())
        }
    }

    pub fn unit_handle_by_index(&self, index: u32) -> Result<unit::Unit> {
        unit::Unit::new(index)
    }
}

impl Drop for NVML {
    fn drop(&mut self) {
        unsafe {
            nvmlShutdown();
        }
    }
}
#[derive(Clone)]
pub struct PCIInfo {
    pub bus_id: String,
    pub bar1: u64,
    pub bandwidth: u64,
}

impl Default for PCIInfo {
    fn default() -> PCIInfo {
        PCIInfo {
            bus_id: "".to_owned(),
            bar1: 0,
            bandwidth: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ClockInfo {
    pub cores: u64,
    pub memory: u64,
}

impl Default for ClockInfo {
    fn default() -> ClockInfo {
        ClockInfo {
            cores: 0,
            memory: 0,
        }
    }
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

#[derive(Debug, Copy, Clone)]
pub struct CudaComputeCapabilityInfo {
    pub major: u64,
    pub minor: u64,
}

#[derive(Clone)]
pub struct Device {
    pub handler: Handler,
    pub uuid: String,
    pub path: String,
    pub model: String,
    pub power: u64,
    pub memory: u64,
    pub cpu_affinity: u64,
    pub pci: PCIInfo,
    pub clocks: ClockInfo,
    pub topology: Vec<i8>,
    pub cuda_compute_capability: CudaComputeCapabilityInfo,
}

impl Default for Device {
    fn default() -> Device {
        unsafe {
            Device {
                handler: std::mem::uninitialized(),
                uuid: String::from(""),
                path: String::from(""),
                model: String::from(""),
                power: 0,
                memory: 0,
                cpu_affinity: 0,
                pci: std::mem::uninitialized(),
                clocks: std::mem::uninitialized(),
                topology: vec![],
                cuda_compute_capability: std::mem::uninitialized(),
            }
        }
    }
}

impl Device {
    pub fn new(index: u32) -> Result<Self> {
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
            power: power,
            memory: memory_info.total,
            cpu_affinity: node,
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

    fn numa_node(bus_id: &str) -> Result<u64> {
        let filepath = format!("/sys/bus/pci/devices/{}/numa_node", bus_id.to_lowercase());
        match std::fs::read_to_string(&filepath) {
            Ok(content) => match content.parse() {
                Ok(node) => Ok(node),
                Err(e) => Err(Error::new(&format!("{}", e))),
            },
            Err(_) => Ok(0),
        }
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

    pub fn get_temperature(&self, dst: DeviceSensorType) -> Result<u64> {
        self.handler.get_temperature(dst)
    }
}

#[derive(Copy, Clone)]
pub struct Handler {
    pub dev: nvmlDevice_t,
}

impl Handler {
    pub fn new(index: u32) -> Result<Handler> {
        unsafe {
            let mut dev: nvmlDevice_t = std::ptr::null_mut();
            let result = nvmlDeviceGetHandleByIndex_v2(
                index as ::std::os::raw::c_uint,
                &mut dev as *mut nvmlDevice_t,
            );
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result.into());
            }
            Ok(Handler { dev })
        }
    }
    pub fn get_name(&self) -> Result<String> {
        unsafe {
            let mut name: [::std::os::raw::c_char; NVML_DEVICE_NAME_BUFFER_SIZE as usize] =
                [0; NVML_DEVICE_NAME_BUFFER_SIZE as usize];
            let result = nvmlDeviceGetName(self.dev, &mut name[0], NVML_DEVICE_NAME_BUFFER_SIZE);
            if result == nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED {
                return Err(Error::new("not supported"));
            }
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(std::ffi::CStr::from_ptr(name.as_ptr() as *const _)
                    .to_str()
                    .unwrap()
                    .to_owned());
            }
            Err(result.into())
        }
    }
    pub fn get_uuid(&self) -> Result<String> {
        unsafe {
            let mut name: [::std::os::raw::c_char; NVML_DEVICE_UUID_BUFFER_SIZE as usize] =
                [0; NVML_DEVICE_UUID_BUFFER_SIZE as usize];
            let result = nvmlDeviceGetUUID(self.dev, &mut name[0], NVML_DEVICE_UUID_BUFFER_SIZE);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(std::ffi::CStr::from_ptr(name.as_ptr() as *const _)
                    .to_str()
                    .unwrap()
                    .to_owned());
            }
            Err(result.into())
        }
    }

    pub fn get_pci_info(&self) -> Result<String> {
        unsafe {
            let mut pci_info: nvmlPciInfo_t = std::mem::uninitialized();
            let result = nvmlDeviceGetPciInfo_v3(self.dev, &mut pci_info as *mut nvmlPciInfo_t);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(
                    std::ffi::CStr::from_ptr(pci_info.busId.as_ptr() as *const _)
                        .to_str()
                        .unwrap()
                        .to_owned(),
                );
            }
            Err(result.into())
        }
    }

    pub fn get_bar1_memory_info(&self) -> Result<(u64, u64)> {
        unsafe {
            let mut bar1_memory_info: nvmlBAR1Memory_t = std::mem::uninitialized();
            let result = nvmlDeviceGetBAR1MemoryInfo(
                self.dev,
                &mut bar1_memory_info as *mut nvmlBAR1Memory_t,
            );
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result.into());
            }
            return Ok((
                bar1_memory_info.bar1Total as u64,
                bar1_memory_info.bar1Used as u64,
            ));
        }
    }

    pub fn get_clock_info(&self) -> Result<(u64, u64)> {
        unsafe {
            let mut sm: ::std::os::raw::c_uint = 0;
            let mut mem: ::std::os::raw::c_uint = 0;
            let result = nvmlDeviceGetClockInfo(
                self.dev,
                nvmlClockType_enum_NVML_CLOCK_SM,
                &mut sm as *mut ::std::os::raw::c_uint,
            );

            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result.into());
            }
            let result = nvmlDeviceGetClockInfo(
                self.dev,
                nvmlClockType_enum_NVML_CLOCK_MEM,
                &mut mem as *mut ::std::os::raw::c_uint,
            );
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result.into());
            }
            return Ok((sm as u64, mem as u64));
        }
    }
    pub fn get_cuda_compute_capability(&self) -> Result<(u64, u64)> {
        unsafe {
            let mut major: ::std::os::raw::c_int = 0;
            let mut minor: ::std::os::raw::c_int = 0;
            let result = nvmlDeviceGetCudaComputeCapability(
                self.dev,
                &mut major as *mut ::std::os::raw::c_int,
                &mut minor as *mut ::std::os::raw::c_int,
            );
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result.into());
            }
            return Ok((major as u64, minor as u64));
        }
    }

    pub fn get_memory_info(&self) -> Result<nvmlMemory_t> {
        unsafe {
            let mut mem: nvmlMemory_t = std::mem::uninitialized();
            let result = nvmlDeviceGetMemoryInfo(self.dev, &mut mem as *mut nvmlMemory_t);
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result.into());
            }
            Ok(mem)
        }
    }

    pub fn get_minor_number(&self) -> Result<u64> {
        self.get_one_interger(nvmlDeviceGetMinorNumber)
    }

    pub fn get_power_management_limit(&self) -> Result<u64> {
        self.get_one_interger(nvmlDeviceGetPowerManagementLimit)
    }

    pub fn get_max_pcie_link_generation(&self) -> Result<u64> {
        self.get_one_interger(nvmlDeviceGetMaxPcieLinkGeneration)
    }

    pub fn get_max_pcie_link_width(&self) -> Result<u64> {
        self.get_one_interger(nvmlDeviceGetMaxPcieLinkWidth)
    }

    pub fn get_temperature(&self, sensor_type: DeviceSensorType) -> Result<u64> {
        unsafe {
            let mut temperature: c_uint = 0;
            let result = nvmlDeviceGetTemperature(
                self.dev,
                sensor_type.into(),
                &mut temperature as *mut c_uint,
            );
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result.into());
            }
            Ok(temperature as u64)
        }
    }
}

pub enum DeviceSensorType {
    GPU,
    COUNT,
}

impl From<DeviceSensorType> for nvmlTemperatureSensors_t {
    fn from(dst: DeviceSensorType) -> nvmlTemperatureSensors_t {
        match dst {
            DeviceSensorType::GPU => nvmlTemperatureSensors_enum_NVML_TEMPERATURE_GPU,
            DeviceSensorType::COUNT => nvmlTemperatureSensors_enum_NVML_TEMPERATURE_COUNT,
        }
    }
}

impl Handler {
    fn get_one_interger(&self, f: ProcessOneInterger) -> Result<u64> {
        unsafe {
            let mut n: ::std::os::raw::c_uint = 0;
            let result = f(self.dev, &mut n as *mut ::std::os::raw::c_uint);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(n as u64);
            }
            Err(result.into())
        }
    }
}
