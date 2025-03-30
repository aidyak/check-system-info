use clap::Parser;
use std::{thread, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 更新間隔：5秒
    #[arg(short, long, default_value_t = 5)]
    interval: u64,

    /// CPU使用率
    #[arg(long)]
    cpu: bool,

    /// メモリ使用率
    #[arg(long)]
    memory: bool,
}

fn main() {
    let args = Args::parse();

    let mut system = System::new_all();

    loop {
        system.refresh_all();

        if args.cpu {
            println!("CPU使用率: {:.2}%", system.global_cpu_info().cpu_usage());
        }

        if args.memory {
            println!(
                "総メモリ量: {} MB, 使用中メモリ量: {} MB, 空きメモリ: {} MB",
                system.total_memory() / 1024,
                system.used_memory() / 1024,
                system.free_memory() / 1024
            );
        }

        println!("---------------------");

        thread::sleep(Duration::from_secs(args.interval));
    }
}
