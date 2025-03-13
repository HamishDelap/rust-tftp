
mod tftp_protocol;
mod client;
mod server;

use std::env;

enum AppMode {
    Client,
    Server,
    Unknown
}

// TFTP server
// - Reads from the server
// - Server on port 2000
// - Client on random port between 2000-2500

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
   
    let mut app_mode = AppMode::Unknown;
    if args.len() > 1 {
        match args[1].clone().to_lowercase().as_str() {
            "client" => app_mode = AppMode::Client,
            "server" => app_mode = AppMode::Server,
            _ => app_mode = AppMode::Unknown
        }
    }

    if matches!(app_mode, AppMode::Client) {
        match client::client::validate_input(args) {
            Some(file_name) => client::client::client_main(file_name)?,
            None => println!("Invalid file_name specified")
        }

    }
    else if matches!(app_mode, AppMode::Server) {
        server::server::server_main()?;
    } 
    else {
        println!("Unknown mode specified");
    }
    
    Ok(())
}

