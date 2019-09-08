use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-cdylib-link-arg=-Wl,--unresolved-symbols=ignore-in-object-files");
    println!("cargo:include=nvml-sys/include");
    println!("cargo:rustc-link-lib=dylib=nvidia-ml");

    let library_directories = vec!["/usr/lib"];
    for library_directory in library_directories {
        if let Ok(entry) = std::fs::read_dir(&library_directory) {
            for dir in entry {
                if let Ok(dir) = dir {
                    let path: std::path::PathBuf = dir.path();
                    if !path.is_dir() {
                        continue;
                    }
                    let path = path.to_str().unwrap();
                    if path.contains("nvidia") {
                        println!("cargo:rustc-link-search=native={}", path);
                    }
                }
            }
        }
    }

    let bindings = bindgen::Builder::default()
        .header("include/nvml.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
