fn main() {
    if std::env::var_os("KERNEL_TEST_MODE").is_some() {
        println!("cargo:rustc-cfg=test");
    }
}