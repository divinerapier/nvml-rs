use crate::error::{Error, Result};

use ::std::os::raw::{c_char, c_int, c_uint};
use nvml_binding::*;

pub struct Unit {
    pub handle: nvmlUnit_t,
}

impl Unit {
    pub fn new(index: u32) -> Result<Unit> {
        unsafe {
            let mut handle: nvmlUnit_t = std::mem::uninitialized();
            let result = nvmlUnitGetHandleByIndex(index as c_uint, &mut handle as *mut nvmlUnit_t);
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Ok(Unit { handle });
            }
            Err(result.into())
        }
    }

    pub fn info(&self) -> Result<nvmlUnitInfo_t> {
        unsafe {
            let mut info = std::mem::uninitialized();
            let result = nvmlUnitGetUnitInfo(self.handle, &mut info as *mut nvmlUnitInfo_t);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(info);
            }
            Err(result.into())
        }
    }
    pub fn led_state(&self) -> Result<nvmlLedState_t> {
        unsafe {
            let mut state = std::mem::uninitialized();
            let result = nvmlUnitGetLedState(self.handle, &mut state as *mut nvmlLedState_t);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(state);
            }
            Err(result.into())
        }
    }
    pub fn psu_info(&self) -> Result<nvmlPSUInfo_t> {
        unsafe {
            let mut psu = std::mem::uninitialized();
            let result = nvmlUnitGetPsuInfo(self.handle, &mut psu as *mut nvmlPSUInfo_t);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(psu);
            }
            Err(result.into())
        }
    }
    pub fn temperature(&self, tt: TemperatureType) -> Result<u64> {
        unsafe {
            let mut temperature: c_uint = 0;
            let result =
                nvmlUnitGetTemperature(self.handle, tt as c_uint, &mut temperature as *mut c_uint);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(temperature as u64);
            }
            Err(result.into())
        }
    }
    pub fn fan_speed(&self) -> Result<nvmlUnitFanSpeeds_t> {
        unsafe {
            let mut fan_speed: nvmlUnitFanSpeeds_t = std::mem::uninitialized();
            let result =
                nvmlUnitGetFanSpeedInfo(self.handle, &mut fan_speed as *mut nvmlUnitFanSpeeds_t);
            if result == nvmlReturn_enum_NVML_SUCCESS {
                return Ok(fan_speed);
            }
            Err(result.into())
        }
    }
    pub fn devices(&self) -> Result<Vec<nvmlDevice_t>> {
        unsafe {
            let mut devices: nvmlDevice_t = std::ptr::null_mut();
            let mut count: c_uint = 0;
            let result = nvmlUnitGetDevices(
                self.handle,
                &mut count as *mut c_uint,
                &mut devices as *mut nvmlDevice_t,
            );
            if result != nvmlReturn_enum_NVML_SUCCESS {
                return Err(result.into());
            }
            Ok(Vec::from_raw_parts(
                &mut devices as *mut nvmlDevice_t,
                count as usize,
                count as usize,
            ))
        }
    }
}

pub enum TemperatureType {
    Intake,
    Exhaust,
    Board,
}

impl From<TemperatureType> for u64 {
    fn from(tt: TemperatureType) -> u64 {
        match tt {
            TemperatureType::Intake => 0,
            TemperatureType::Exhaust => 1,
            TemperatureType::Board => 2,
        }
    }
}
