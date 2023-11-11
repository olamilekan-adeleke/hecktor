use std::process::Command;
use std::{error::Error, path::Path};
use std::{io, process};

pub use config::Config;

mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Checking if in a flutter project...");

    let path: &str = config.path.as_str();
    check_if_in_flutter_project(path).unwrap_or_else(|err| {
        eprintln!("Error: Not a flutter project\n{}", err);
        process::exit(1);
    });

    println!("flutter project found...");
    println!("🚀 Running flutter build...");
    run_flutter_build(config).unwrap_or_else(|err| {
        eprintln!("⚠️ Error: Failed to run flutter build..\n{}", err);
        process::exit(1);
    });
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

fn run_flutter_build(config: Config) -> Result<(), Box<dyn Error>> {
    let mut run_flutter = Command::new("flutter")
        .current_dir(config.path)
        .arg("build")
        .arg("apk")
        .spawn()?;

    println!("{:?}", run_flutter);
    run_flutter.wait()?;

    Ok(())
}
