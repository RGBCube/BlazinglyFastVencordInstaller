use crate::discord;

use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use nix::unistd::Uid;
use users::os::unix::UserExt;

const LINUX_DISCORD_NAMES: [&str; 15] = [
    "Discord",
    "DiscordPTB",
    "DiscordCanary",
    "DiscordDevelopment",
    "discord",
    "discordptb",
    "discordcanary",
    "discorddevelopment",
    "discord-ptb",
    "discord-canary",
    "discord-development",
    // Flatpak
    "com.discordapp.Discord",
    "com.discordapp.DiscordPTB",
    "com.discordapp.DiscordCanary",
    "com.discordapp.DiscordDevelopment",
];

fn get_sudo_user() -> anyhow::Result<String> {
    if let Ok(user) = env::var("SUDO_USER") {
        return Ok(user);
    }

    if let Ok(user) = env::var("DOAS_USER") {
        return Ok(user);
    }

    Err(anyhow!(
        r#"VencordInstaller was run as root but neither the
SUDO_USER or DOAS_USER environment variables are set.
Please rerun as a normal user, with sudo/doas, or manually
set SUDO_USER to your username"#
    ))
}

fn parse_discord_install(mut path: PathBuf) -> Option<discord::Installation> {
    if path.iter().any(|p| p == "flatpak") {
        let mut discord_name = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .strip_prefix("com.discordapp.")
            .unwrap()
            .to_lowercase();

        if discord_name != "discord" {
            // discordcanary -> discord-canary, discordpbt -> discord-pbt, ...
            discord_name.insert(8, '-');
        }

        path = path.join("current/active/files").join(discord_name);
    };

    let resources = path.join("resources");
    let app_path = path.join("app");

    let (is_patched, is_sys_electron) = if resources.exists() {
        // Normal install.
        (app_path.exists() || resources.join("app.asar").is_dir(), false)
    } else if path.join("app.asar").exists() {
        (path.join("_app.asar.unpacked").exists(), true)
    } else {
        return None;
    };

    Some(discord::Installation {
        branch: crate::discord::get_branch(&path).to_string(),
        path,
        app_path,
        is_patched,
        is_sys_electron,
        is_openasar: false,
    })
}

pub fn find() -> anyhow::Result<Vec<discord::Installation>> {
    let home_dir = if Uid::effective().is_root() {
        let real_user_name = get_sudo_user()?;

        if real_user_name == "root" {
            return Err(anyhow!(
                r#"VencordInstaller must not be run as the root user.
Please rerun as normal user. Use sudo or doas to run as root."#
            ));
        }

        users::get_user_by_name(&real_user_name).unwrap().home_dir().to_path_buf()
    } else {
        Path::new(&env::var("HOME")?).to_path_buf()
    };

    let packages = vec![
        Path::new("/usr/share").to_path_buf(),
        Path::new("/usr/lib64").to_path_buf(),
        Path::new("/opt").to_path_buf(),
        Path::new("/var/lib/flatpak/app").to_path_buf(),
        home_dir.join("/.local/share"),
        home_dir.join("/.dvm"),
        home_dir.join("/.local/share/flatpak/app"),
    ];

    let packages = packages
    .iter()
    .filter_map(|package_dir| package_dir.read_dir().ok())
    .flatten()
    .flatten()
    .map(|entry| entry.path());

    let mut discord_installs = vec![];

    for package in packages {
        if package.is_dir() || !LINUX_DISCORD_NAMES.contains(&package.file_name().unwrap().to_str().unwrap())
        {
            continue;
        }

        if let Some(discord_install) = parse_discord_install(package) {
            discord_installs.push(discord_install);
        }
    }

    Ok(discord_installs)
}
