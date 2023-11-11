use std::process::Command;
use std::{error::Error, path::Path};
use std::{io, process, fs};

pub use config::Config;

mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Checking if in a Flutter project...");

    let path: &str = config.path.as_str();
    check_if_in_flutter_project(path).unwrap_or_else(|err| {
        eprintln!("Error: Not a Flutter project\n{}", err);
        process::exit(1);
    });

    println!("Flutter project found...");
    println!("🚀 Running Flutter build...");
    run_flutter_build(&config).unwrap_or_else(|err| {
        eprintln!("⚠️ Error: Failed to run Flutter build..\n{}", err);
        process::exit(1);
    });

    println!("\n✅ Flutter build completed...");
    println!("Opening apk path...");
    open_file_path(path)?;

    println!("\nGetting File");
    let full_path = format!("{}/build/app/outputs/flutter-apk/app-release.apk", path);
    let file = fs::read(full_path)?;
    println!("{:?}", String::from_utf8_lossy(&file));


    Ok(())
}

fn check_if_in_flutter_project(path: &str) -> io::Result<()> {
    let new_path = format!("{}/pubspec.yaml", path);
    let new_path = new_path.as_str();
    if Path::new(new_path).exists() {
        Ok(())
    } else {
        let msg = format!(
            "⚠️  Could not find 'pubspec.yaml' in current directory {}",
            new_path
        );
        Err(io::Error::new(io::ErrorKind::NotFound, msg))
    }
}

fn run_flutter_build(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut run_flutter = Command::new("flutter")
        .current_dir(config.path.clone())
        .arg("build")
        .arg("apk")
        .spawn()?;

    run_flutter.wait()?;
    Ok(())
}

fn open_file_path(path: &str) -> Result<(), Box<dyn Error>> {
    let mut run_open_path = Command::new("open")
        .current_dir(path)
        .arg("./build/app/outputs/flutter-apk/")
        .spawn()?;

    run_open_path.wait()?;
    Ok(())
}
