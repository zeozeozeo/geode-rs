use std::{fs, path::PathBuf, process};

use fat_macho::FatWriter;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <arm64_dylib> <x86_64_dylib> <output>", args[0]);
        eprintln!();
        eprintln!("Combines two thin Mac dylibs (aarch64 + x86_64) into a universal fat binary.");
        eprintln!();
        eprintln!("Example:");
        eprintln!(
            "  {} target/aarch64-apple-darwin/release/mykoollib.dylib \\",
            args[0]
        );
        eprintln!("     target/x86_64-apple-darwin/release/mykoollib.dylib \\");
        eprintln!("     mykoollib.dylib");
        process::exit(1);
    }

    let arm64_path = PathBuf::from(&args[1]);
    let x86_64_path = PathBuf::from(&args[2]);
    let output_path = PathBuf::from(&args[3]);

    let arm64_data = fs::read(&arm64_path).unwrap_or_else(|e| {
        eprintln!(
            "Error reading arm64 dylib '{}': {}",
            arm64_path.display(),
            e
        );
        process::exit(1);
    });

    let x86_64_data = fs::read(&x86_64_path).unwrap_or_else(|e| {
        eprintln!(
            "Error reading x86_64 dylib '{}': {}",
            x86_64_path.display(),
            e
        );
        process::exit(1);
    });

    let mut fat = FatWriter::new();

    fat.add(arm64_data).unwrap_or_else(|e| {
        eprintln!("Error adding arm64 slice: {}", e);
        process::exit(1);
    });

    fat.add(x86_64_data).unwrap_or_else(|e| {
        eprintln!("Error adding x86_64 slice: {}", e);
        process::exit(1);
    });

    fat.write_to_file(&output_path).unwrap_or_else(|e| {
        eprintln!(
            "Error writing universal binary to '{}': {}",
            output_path.display(),
            e
        );
        process::exit(1);
    });

    println!("Created universal binary: {}", output_path.display());
    println!("  arm64  <- {}", arm64_path.display());
    println!("  x86_64 <- {}", x86_64_path.display());
}
