use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};
use std::env;
use std::process::{Command, exit};

const USAGE: &str = "Usage: cargo run -- [uefi|bios]";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BootMode {
    Uefi,
    Bios,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CliAction {
    Run(BootMode),
    Help,
}

fn parse_args(args: &[String]) -> Result<CliAction, &'static str> {
    match args {
        [] => Err("missing boot mode"),
        [arg] if arg.eq_ignore_ascii_case("-h") || arg.eq_ignore_ascii_case("--help") => {
            Ok(CliAction::Help)
        }
        [arg] if arg.eq_ignore_ascii_case("uefi") => Ok(CliAction::Run(BootMode::Uefi)),
        [arg] if arg.eq_ignore_ascii_case("bios") => Ok(CliAction::Run(BootMode::Bios)),
        [_] => Err("unknown boot mode; expected 'uefi' or 'bios'"),
        _ => Err("expected exactly one argument"),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mode = match parse_args(&args) {
        Ok(CliAction::Run(mode)) => mode,
        Ok(CliAction::Help) => {
            println!("{USAGE}");
            return;
        }
        Err(message) => {
            eprintln!("rustalis: {message}\n{USAGE}");
            exit(2);
        }
    };

    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.args(["-serial", "mon:stdio", "-display", "none"]);

    match mode {
        BootMode::Uefi => {
            let prebuilt = Prebuilt::fetch(Source::LATEST, "target/ovmf").unwrap_or_else(|error| {
                eprintln!(
                    "rustalis: failed to obtain OVMF firmware ({error:?}).                      Check network access or the cached firmware in target/ovmf."
                );
                exit(1);
            });
            let code = prebuilt.get_file(Arch::X64, FileType::Code);
            let vars = prebuilt.get_file(Arch::X64, FileType::Vars);
            cmd.arg("-drive")
                .arg(format!("format=raw,file={uefi_path}"));
            cmd.arg("-drive").arg(format!(
                "if=pflash,format=raw,unit=0,file={},readonly=on",
                code.display()
            ));
            cmd.arg("-drive").arg(format!(
                "if=pflash,format=raw,unit=1,file={},snapshot=on",
                vars.display()
            ));
        }
        BootMode::Bios => {
            cmd.arg("-drive")
                .arg(format!("format=raw,file={bios_path}"));
        }
    }

    let status = cmd.status().unwrap_or_else(|error| {
        eprintln!(
            "rustalis: failed to start qemu-system-x86_64: {error}.              Install QEMU and ensure qemu-system-x86_64 is on PATH."
        );
        exit(1);
    });
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

#[cfg(test)]
mod tests {
    use super::{BootMode, CliAction, parse_args};

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn accepts_bios_and_uefi_modes_case_insensitively() {
        assert_eq!(
            parse_args(&args(&["uefi"])),
            Ok(CliAction::Run(BootMode::Uefi))
        );
        assert_eq!(
            parse_args(&args(&["BIOS"])),
            Ok(CliAction::Run(BootMode::Bios))
        );
    }

    #[test]
    fn accepts_help_flags() {
        assert_eq!(parse_args(&args(&["--help"])), Ok(CliAction::Help));
        assert_eq!(parse_args(&args(&["-h"])), Ok(CliAction::Help));
    }

    #[test]
    fn rejects_missing_unknown_and_extra_arguments() {
        assert_eq!(parse_args(&args(&[])), Err("missing boot mode"));
        assert_eq!(
            parse_args(&args(&["pxe"])),
            Err("unknown boot mode; expected 'uefi' or 'bios'")
        );
        assert_eq!(
            parse_args(&args(&["uefi", "extra"])),
            Err("expected exactly one argument")
        );
    }
}
