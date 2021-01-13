use clap::{App, Arg};
use sysinfo::{System, SystemExt};

const SIZE_UNIT: f64 = 1024.0;

fn get_size_str(mut val: f64) -> String {
    for suffix in ["KB", "MB", "GB"].iter() {
        if val < SIZE_UNIT {
            return format!("{:.2}{}", val, suffix);
        }
        val /= SIZE_UNIT;
    }
    format!("{:.2}TB", val)
}

fn main() {
    let mut sys = System::new();

    let matches = App::new("sysinfo-cli")
        .version("0.1.0")
        .arg(
            Arg::with_name("memory")
                .short("m")
                .long("memory")
                .help("Get memory message"),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Get name message"),
        )
        .get_matches();
    if matches.is_present("memory") {
        sys.refresh_memory();
        let free = sys.get_available_memory() as f64;
        let total = sys.get_total_memory() as f64;
        println!(
            "{:<10} {}\navailable: {}",
            "total:",
            get_size_str(total),
            get_size_str(free)
        );
        return;
    }
    if matches.is_present("name") {
        if let (Some(name), Some(version), Some(host_name)) =
            (sys.get_name(), sys.get_version(), sys.get_host_name())
        {
            println!(
                "{:<10} {} {}\nhost_name: {}",
                "os_name:", name, version, host_name
            );
        }
        return;
    }
}
