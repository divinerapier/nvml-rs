fn main() {
    let driver = nvml_rs::NVML::new().unwrap();
    let count = driver.device_count().unwrap();
    let version = driver.driver_version().unwrap();
    println!("version = {}", version);

    for i in 0..count {
        match nvml_rs::Device::new(i) {
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
                println!(
                    "GPU Temperature: {:?}",
                    device.get_temperature(nvml_rs::DeviceSensorType::GPU)
                );
                println!(
                    "COUNT Temperature: {:?}",
                    device.get_temperature(nvml_rs::DeviceSensorType::COUNT)
                );
            }
            Err(message) => println!("error message: {:?}", message),
        }
    }
    let unit_count = driver.unit_count().unwrap();
    println!("Unit Count: {}", unit_count);
}
