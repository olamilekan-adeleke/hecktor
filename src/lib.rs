use std::process::Command;
use std::{error::Error, path::Path};
use std::{fs, io, process};

use google_drive3::{hyper, hyper_rustls, oauth2, DriveHub};

pub use config::Config;

mod config;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
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
    // println!("Requesting for upload permission...");
    // let token = google_authenticate().await.unwrap_or_else(|err| {
    //     eprintln!("Error: Authorization Failed\n{}", err);
    //     process::exit(1);
    // });

    println!("\n Uploading File to google drive");
    let full_path = format!("{}/build/app/outputs/flutter-apk/app-release.apk", path);
    upload_file_to_drive(&full_path.as_str()).await?;

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

// async fn google_drive_authenticate(
// ) -> Result<(), Box<dyn Error>> {
//     let secret = oauth2::read_application_secret("client_secret.json").await?;
//     let auth = oauth2::InstalledFlowAuthenticator::builder(
//         secret,
//         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
//     )
//     .build()
//     .await?;

//     let connector = hyper_rustls::HttpsConnectorBuilder::new()
//         .with_native_roots()
//         .https_or_http()
//         .enable_http1()
//         .build();
//     let hub = DriveHub::new(hyper::Client::builder().build(connector), auth);

//     Ok(hub);
// }

async fn upload_file_to_drive(full_path: &str) -> Result<(), Box<dyn Error>> {
    let secret = oauth2::read_application_secret("client_secret.json").await?;
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await?;

    let hub = DriveHub::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
    );

    let file_metadata = google_drive3::api::File {
        name: Some("apk-ri".to_string()),
        ..Default::default()
    };

    let upload_file = match fs::File::open(Path::new(full_path)) {
        Ok(f) => f,
        Err(err) => {
            panic!("Could not open file: {}", err);
        }
    };

    // Create a request to create a new file
    let (_resp, _file) = hub
        .files()
        .create(file_metadata)
        .upload_resumable(
            upload_file,
            "application/vnd.android.package-archive".parse().unwrap(),
        )
        .await?;

    Ok(())
}
