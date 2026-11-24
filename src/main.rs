use anyhow::{Result, bail};
use clap::{command, arg};

use jl::JLC;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = command!()
        .arg(
            arg!(-u --user <USER> "Username for authentication")
        )
        .arg(
            arg!(-p --password <PASSWORD> "Password for authentication")
        )
        .arg(
            arg!(-H --host <HOST> "Host of the j-lawyer-server")
        )
        .arg(
            arg!(-P --port <PORT> "Port of the j-lawyer-server")
        )
        .arg(
            arg!(-s --ssl "Use SSL for the connection (https instead of http)")
        )
        .get_matches();

    let Some(user) = matches.get_one::<String>("user") else {
        bail!("User is required");
    };
    let Some(password) = matches.get_one::<String>("password") else {
        bail!("Password is required");
    };
    let Some(host) = matches.get_one::<String>("host") else {
        bail!("Host is required");
    };
    let Some(port) = matches.get_one::<String>("port") else {
        bail!("Port is required");
    };
    let ssl = matches.get_flag("ssl");

    let jlc = JLC::new(user.to_string(), password.to_string(), host.to_string(), port.to_string(), ssl);
    let response = jlc.execute().await?;

    println!("{response:?}");

    let text = response.text().await?;
    println!("{text:?}");

    Ok(())
}
