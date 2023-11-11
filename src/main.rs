// use std::process;

use std::error::Error;

use google_drive3::oauth2;
// use hecktor::*;

// async fn run() -> Result<(), Box<dyn Error>> {
// let args: Vec<String> = std::env::args().collect();
// let config = Config::new(&args).unwrap_or_else(|err| {
//     eprintln!("-------- Problem parsing arguments -------- \n{}", err);
//     process::exit(1);
// });
// if let Err(err) = run(config) {
//     eprintln!("Application stop due to error: {}", err);
//     process::exit(1);
// }
// Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    authenticate_with_google_drive().await?;

    Ok(())
}

//
async fn authenticate_with_google_drive() -> Result<(), Box<dyn std::error::Error>> {
    // Load your OAuth 2.0 credentials from a file
    let secret = oauth2::read_application_secret("client_secret.json").await?;

    // Create an authenticator that uses the client secret to authenticate
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await?;

    // Request access to Google Drive
    let token = auth
        .token(&["https://www.googleapis.com/auth/drive.file"])
        .await?;

    // Return the access token
    println!("{:?}", token);
    Ok(())
}
