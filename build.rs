use std::path::PathBuf;
use std::env;


fn main() {
    let out_directory = PathBuf::from(
        env::var_os("OUT_DIR")
            .expect("Environment variable `OUT_DIR` should always be present when `build.rs` is being executed")
    );

    let kernel_path = PathBuf::from(
        env::var_os("CARGO_BIN_FILE_RUST_OS_KERNEL")
            .expect("Environment variable `CARGO_BIN_FILE_RUST_OS_KERNEL` should be present when the `kernel` crate is an artifact build dependency")
    );

    let uefi_path = out_directory.join("uefi.img");
    bootloader::UefiBoot::new(&kernel_path)
        .create_disk_image(&uefi_path)
        .unwrap();

    let bios_path = out_directory.join("bios.img");
    bootloader::BiosBoot::new(&kernel_path)
        .create_disk_image(&bios_path)
        .unwrap();

    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}