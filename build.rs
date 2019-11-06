use std::{env, fs};
use std::path::PathBuf;

fn main() {
    // Put the memory definitions somewhere the linker can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    let boards: Vec<_> = env::vars().filter_map(|(key, _value)| {
        if key.starts_with("CARGO_FEATURE_BOARD") {
            Some(key[20..].to_ascii_lowercase())  // Strip 'CARGO_FEATURE_BOARD_'
        } else {
            None
        }
    }).collect();

    if boards.is_empty() {
        panic!("No board features selected");
    }
    if boards.len() > 1 {
        panic!("More than one board feature selected: {:?}", boards);
    }

    let board = boards.first().unwrap();

    match board.as_str() {
        "hifive1" => {
            fs::copy("memory-hifive1.x", out_dir.join("hifive1-memory.x")).unwrap();
            println!("cargo:rerun-if-changed=memory-hifive1.x");
        }
        "hifive1_revb" => {
            fs::copy("memory-hifive1-revb.x", out_dir.join("hifive1-memory.x")).unwrap();
            println!("cargo:rerun-if-changed=memory-hifive1-revb.x");
        }
        "lofive" => {}
        "lofive_r1" => {
            fs::copy("memory-lofive-r1.x", out_dir.join("hifive1-memory.x")).unwrap();
            println!("cargo:rerun-if-changed=memory-lofive-r1.x");
        }

        other => panic!("Unknown board: {}", other),
    }

    fs::copy("hifive1-link.x", out_dir.join("hifive1-link.x")).unwrap();
}
