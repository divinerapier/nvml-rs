#[derive(Debug)]
pub struct Error {
    message: Option<String>,
}

impl Default for Error {
    fn default() -> Error {
        Error { message: None }
    }
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error {
            message: Some(message.into()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<nvml_binding::nvmlReturn_t> for Error {
    fn from(r: nvml_binding::nvmlReturn_t) -> Error {
        unsafe {
            if r == nvml_binding::nvmlReturn_enum_NVML_SUCCESS {
                return Error { message: None };
            }
            let ptr = nvml_binding::nvmlErrorString(r);
            let message = std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_owned();
            Error {
                message: Some(message),
            }
        }
    }
}
