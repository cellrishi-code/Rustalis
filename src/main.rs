use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};
use std::env;
use std::process::{exit, Command};

fn main() {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");
    let args: Vec<String> = env::args().collect();

    let uefi = match args.get(1).map(|s| s.to_lowercase()).as_deref() {
        Some("uefi") => true,
        Some("bios") => false,
        Some("-h") | Some("--help") => {
            println!("Usage: cargo run -- [uefi|bios]");
            exit(0);
        }
        _ => {
            eprintln!("Usage: cargo run -- [uefi|bios]");
            exit(1);
        }
    };

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.args(["-serial", "mon:stdio", "-display", "none"]);

    if uefi {
        let prebuilt = Prebuilt::fetch(Source::LATEST, "target/ovmf")
            .expect("failed to obtain OVMF firmware");
        let code = prebuilt.get_file(Arch::X64, FileType::Code);
        let vars = prebuilt.get_file(Arch::X64, FileType::Vars);
        cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
        cmd.arg("-drive").arg(format!("if=pflash,format=raw,unit=0,file={},readonly=on", code.display()));
        cmd.arg("-drive").arg(format!("if=pflash,format=raw,unit=1,file={},snapshot=on", vars.display()));
    } else {
        cmd.arg("-drive").arg(format!("format=raw,file={bios_path}"));
    }

    let status = cmd.status().expect("failed to start qemu-system-x86_64");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}
