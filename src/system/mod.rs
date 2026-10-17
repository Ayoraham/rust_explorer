use colored::Colorize;
use sysinfo::{Disks, System};

const GB: f64 = 1_073_741_824.0; // 1024^3

fn to_gb(bytes: u64) -> f64 {
    bytes as f64 / GB
}

fn usage_bar(used: u64, total: u64, width: usize) -> String {
    if total == 0 {
        return "░".repeat(width);
    }
    let ratio = used as f64 / total as f64;
    let filled = ((ratio * width as f64).round() as usize).min(width);
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(width - filled));

    // colour by how full it is
    if ratio < 0.60 {
        bar.green().to_string()
    } else if ratio < 0.85 {
        bar.yellow().to_string()
    } else {
        bar.red().to_string()
    }
}

fn section(title: &str) {
    println!("\n{}", format!("── {title} ").bold().cyan());
}

pub fn get_sysinfo() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // ---------- System ----------
    section("System");
    println!(
        "{:<14}{}",
        "OS:",
        System::name().unwrap_or("Unknown".into())
    );
    println!(
        "{:<14}{}",
        "Version:",
        System::os_version().unwrap_or("Unknown".into())
    );
    println!(
        "{:<14}{}",
        "Kernel:",
        System::kernel_version().unwrap_or("Unknown".into())
    );
    println!(
        "{:<14}{}",
        "Host:",
        System::host_name().unwrap_or("Unknown".into())
    );

    // ---------- CPU ----------
    section("CPU");
    let brand = sys.cpus().first().map(|c| c.brand()).unwrap_or("Unknown");
    println!("{:<14}{}", "Processor:", brand.trim());
    println!("{:<14}{}", "Logical cores:", sys.cpus().len());

    // ---------- Memory ----------
    section("Memory");
    let total = sys.total_memory();
    let used = sys.used_memory();
    let available = sys.available_memory();

    println!("{:<14}{:.2} GB", "Total:", to_gb(total));
    println!("{:<14}{:.2} GB", "Used:", to_gb(used));
    println!("{:<14}{:.2} GB", "Available:", to_gb(available));
    println!(
        "{} {:.1}%",
        usage_bar(used, total, 30),
        used as f64 / total as f64 * 100.0
    );

    // ---------- Disks ----------
    section("Storage");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total = disk.total_space();
        if total == 0 {
            continue;
        }
        let free = disk.available_space();
        let used = total - free;

        println!(
            "{} {}",
            disk.mount_point().display().to_string().bold(),
            format!("({})", disk.file_system().to_string_lossy()).dimmed()
        );
        println!(
            "  {:.1} GB used / {:.1} GB total, {:.1} GB free",
            to_gb(used),
            to_gb(total),
            to_gb(free)
        );
        println!(
            "  {} {:.1}%",
            usage_bar(used, total, 30),
            used as f64 / total as f64 * 100.0
        );
    }
    println!();
}
