use std::env;
use std::path::PathBuf;

use ld_memory::{Memory, MemorySection};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let memory = Memory::new()
        .add_section(MemorySection::new("FLASH", 0x00000000, 0x2000))
        .add_section(MemorySection::new("RAM", 0x20000000, 0x2000));

    let mut memoryx = PathBuf::from(&out_dir);
    memoryx.push("memory.x");

    std::fs::write(memoryx, memory.to_string()).unwrap();
    println!("cargo:rustc-link-search=native={}", &out_dir);
}
