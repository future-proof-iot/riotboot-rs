use std::env;
use std::path::PathBuf;

use ld_memory::{Memory, MemorySection};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let flash_start =
        env::var("RIOTBOOT_FLASH_START").map_or(0usize, |env| env.parse::<usize>().unwrap());

    let flash_len =
        env::var("RIOTBOOT_FLASH_LEN").map_or(0x2000usize, |env| env.parse::<usize>().unwrap());

    let ram_start =
        env::var("RIOTBOOT_RAM_START").map_or(0x20000000usize, |env| env.parse::<usize>().unwrap());

    let ram_len =
        env::var("RIOTBOOT_RAM_LEN").map_or(0x2000usize, |env| env.parse::<usize>().unwrap());

    let memory = Memory::new()
        .add_section(MemorySection::new("FLASH", flash_start, flash_len))
        .add_section(MemorySection::new("RAM", ram_start, ram_len));

    let mut memoryx = PathBuf::from(&out_dir);
    memoryx.push("memory.x");

    std::fs::write(memoryx, memory.to_string()).unwrap();
    println!("cargo:rerun-if-env-changed=RIOTBOOT_FLASH_START");
    println!("cargo:rerun-if-env-changed=RIOTBOOT_FLASH_LEN");
    println!("cargo:rerun-if-env-changed=RIOTBOOT_RAM_START");
    println!("cargo:rerun-if-env-changed=RIOTBOOT_RAM_LEN");
    println!("cargo:rustc-link-search=native={}", &out_dir);
}
