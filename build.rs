fn main() {
    if cfg!(target_os = "windows") {
        // Additional linker arguments required for building Windows targets.
        println!("cargo:rustc-link-arg-bins=/ENTRY:_start");
        println!("cargo:rustc-link-arg-bins=/SUBSYSTEM:console");
    }

    if cfg!(target_os = "linux") {
        // Additional linker arguments required for building Linux targets.
        println!("cargo:rustc-link-arg-bins=-nostartfiles");
    }

    if cfg!(target_os = "macos") {
        // Additional linker arguments required for building MacOS targets.
        println!("cargo:rustc-link-arg-bins=-e");
        println!("cargo:rustc-link-arg-bins=__start");
        println!("cargo:rustc-link-arg-bins=-static");
        println!("cargo:rustc-link-arg-bins=-nostartfiles");
    }
}