use std::env;
use std::path::PathBuf;

use ld_memory::{Memory, MemorySection};

fn parse_usize(string: &str) -> Result<usize, std::num::ParseIntError> {
    if string.starts_with("0x") {
        let trimmed = string.trim_start_matches("0x");
        usize::from_str_radix(trimmed, 16)
    } else {
        string.parse::<usize>()
    }
}

fn parse_from_env(var: &str, default: usize) -> Result<usize, std::num::ParseIntError> {
    if let Ok(val) = env::var(var) {
        Ok(parse_usize(&val)?)
    } else {
        Ok(default)
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let flash_start = parse_from_env("RIOTBOOT_FLASH_START", 0).unwrap();
    let flash_len = parse_from_env("RIOTBOOT_FLASH_LEN", 0x2000usize).unwrap();
    let ram_start = parse_from_env("RIOTBOOT_RAM_START", 0x20000000usize).unwrap();
    let ram_len = parse_from_env("RIOTBOOT_RAM_LEN", 0x2000usize).unwrap();

    eprintln!("{} {} {} {}", flash_start, flash_len, ram_start, ram_len);

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
    println!("cargo:rustc-link-arg=-Tmemory.x");
}
