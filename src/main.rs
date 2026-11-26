use anyhow::{Result, bail};
use clap::{arg, command, value_parser};
use directories::ProjectDirs;

mod profile;
use profile::Profile;

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
                .value_parser(value_parser!(u32))
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
    let Some(port) = matches.get_one::<u32>("port") else {
        bail!("Port is required");
    };
    let ssl = matches.get_flag("ssl");

    let proj_dirs = ProjectDirs::from("de", "keienborg", "jlctl");
    let conf_dir = if let Some(proj_dirs) = proj_dirs {
       Some(proj_dirs.config_dir().to_owned())
    } else {
       None
    };

    let profile = Profile::new(user.to_string(), password.to_string(), host.to_string(), *port, ssl);
    let jlc = JLC::new(profile.user, profile.password, profile.host, profile.port, profile.ssl);
    let response = jlc.execute().await?;

    println!("{response:?}");

    let text = response.text().await?;
    println!("{text:?}");

    println!();
    println!("Configuration directory: {:?}", conf_dir);

    Ok(())
}
