use sysinfo::{System, SystemExt, ProcessorExt, DiskExt, ProcessExt};

fn main() {
    // Membuat objek sistem untuk mendapatkan informasi
    let mut system = System::new_all();

    // Memperbarui informasi sistem
    system.refresh_all();

    // Menampilkan informasi penggunaan CPU
    println!("CPU Usage: {}%", system.global_processor_info().cpu_usage());

    // Menampilkan informasi penggunaan Memori
    println!("Total Memory: {} MB", system.total_memory() / 1024);
    println!("Used Memory: {} MB", system.used_memory() / 1024);
    println!("Free Memory: {} MB", system.free_memory() / 1024);

    // Menampilkan informasi tentang disk
    for disk in system.disks() {
        println!("Disk: {:?}", disk.name());
        println!("Total Disk: {} MB", disk.total_space() / 1024 / 1024);
        println!("Used Disk: {} MB", (disk.total_space() - disk.available_space()) / 1024 / 1024);
        println!("Free Disk: {} MB", disk.available_space() / 1024 / 1024);
    }

    // Menampilkan informasi semua proses yang sedang berjalan
    for process in system.processes() {
        println!("Process: {} PID: {}", process.1.name(), process.0);
    }
}
