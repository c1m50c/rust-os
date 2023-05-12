#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

use std::io::Error;
use std::process;

use clap::Parser;


#[derive(Debug, clap::Parser)]
pub struct AppArguments {
    #[clap(subcommand)]
    pub action: LaunchAction,
}


#[derive(Debug, clap::Subcommand)]
pub enum LaunchAction {
    /// Starts the `kernel` UEFI image in QEMU.
    Uefi,

    /// Starts the `kernel` BIOS image in QEMU.
    Bios,
}


fn main() -> Result<(), Error> {
    let mut command = process::Command::new("qemu-system-x86_64");
    let app_arguments = AppArguments::parse();

    match app_arguments.action {
        LaunchAction::Uefi => {
            command.arg("-bios")
                .arg( ovmf_prebuilt::ovmf_pure_efi() );
            command.arg("-drive")
                .arg( format!("format=raw,file={}", env!("UEFI_PATH")) );
        },

        LaunchAction::Bios => {
            command.arg("-drive")
                .arg( format!("format=raw,file={}", env!("BIOS_PATH")) );
        },
    }

    #[cfg(feature = "testing")] {
        command.arg("-device")
            .arg("isa-debug-exit,iobase=0xf4,iosize=0x04");
        command.arg("-serial")
            .arg("stdio");
        command.arg("-display")
            .arg("none");
    }

    let mut child_process = command.spawn()?;
    child_process.wait()?;

    Ok(())
}