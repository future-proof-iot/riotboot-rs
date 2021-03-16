use std::env;
use std::path::Path;

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
    let constants_path = Path::new(&out_dir).join("constants.rs");

    let flash_start = parse_from_env("RIOTBOOT_FLASH_START", 0).unwrap();
    let flash_len = parse_from_env("RIOTBOOT_FLASH_LEN", 0x2000usize).unwrap();
    let ram_start = parse_from_env("RIOTBOOT_RAM_START", 0x20000000usize).unwrap();
    let ram_len = parse_from_env("RIOTBOOT_RAM_LEN", 0x2000usize).unwrap();

    let slot0_offset = parse_from_env("RIOTBOOT_SLOT0_OFFSET", 0).unwrap();
    let slot1_offset = parse_from_env("RIOTBOOT_SLOT1_OFFSET", 0).unwrap();

    let mut constants = String::new();
    constants.push_str(&format!(
        "const SLOT0_ADDR: usize = {};\n",
        flash_start + slot0_offset
    ));

    constants.push_str(&format!(
        "const SLOT1_ADDR: usize = {};\n",
        flash_start + slot1_offset
    ));
    std::fs::write(constants_path, constants).unwrap();

    let memory = Memory::new()
        .add_section(MemorySection::new("FLASH", flash_start, flash_len))
        .add_section(MemorySection::new("RAM", ram_start, ram_len));

    let memoryx = Path::new(&out_dir).join("memory.x");

    std::fs::write(memoryx, memory.to_string()).unwrap();
    println!("cargo:rerun-if-env-changed=RIOTBOOT_FLASH_START");
    println!("cargo:rerun-if-env-changed=RIOTBOOT_FLASH_LEN");
    println!("cargo:rerun-if-env-changed=RIOTBOOT_RAM_START");
    println!("cargo:rerun-if-env-changed=RIOTBOOT_RAM_LEN");
    println!("cargo:rerun-if-env-changed=RIOTBOOT_SLOT0_OFFSET");
    println!("cargo:rerun-if-env-changed=RIOTBOOT_SLOT1_OFFSET");
    println!("cargo:rustc-link-search=native={}", &out_dir);
    println!("cargo:rustc-link-arg=-Tlink.x");
}
