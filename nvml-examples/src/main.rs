fn main() {
    let driver = nvml::NVML::new().unwrap();
    let count = driver.device_get_count().unwrap();
    let version = driver.nvml_system_get_driver_version().unwrap();
    println!("version = {}", version);

    for i in 0..count {
        match nvml::Device::new(i) {
            Ok(device) => {
                println!("UUID: {}", device.uuid);
                println!("Model: {}", device.model);
                println!("Path: {}", device.path);
                println!("Power: {}", device.power);
                println!("Memory: {}", device.memory);
                println!("CudaComputeCap: {:?}", device.cuda_compute_capability);
                println!("CPU Affinity: {}", device.cpu_affinity);
                println!("Bus ID: {}", device.pci.bus_id);
                println!("BAR1: {}", device.pci.bar1);
                println!("Bandwidth: {}", device.pci.bandwidth);
                println!("Cores: {}", device.clocks.cores);
                println!("Memory: {}", device.clocks.memory);
                println!("P2P Available: {:?}", device.topology);
            }
            Err(message) => println!("error message: {}", message),
        }
    }
}
