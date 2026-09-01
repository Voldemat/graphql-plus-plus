use crate::github;
use crate::platform;

fn install_dir() -> Result<std::path::PathBuf, String> {
    let home = std::env::var_os("HOME")
        .ok_or_else(|| "HOME environment variable is not set".to_string())?;
    Ok(std::path::PathBuf::from(home).join(".gqlup").join("bin"))
}

fn version_file(install_dir: &std::path::Path) -> std::path::PathBuf {
    install_dir.join(".gql-version")
}

fn binary_path(install_dir: &std::path::Path) -> std::path::PathBuf {
    install_dir.join("gql")
}

fn installed_version(install_dir: &std::path::Path) -> Option<String> {
    std::fs::read_to_string(version_file(install_dir))
        .ok()
        .map(|s| s.trim().to_string())
}

fn write_binary(
    install_dir: &std::path::Path,
    bytes: &[u8],
) -> Result<(), String> {
    std::fs::create_dir_all(install_dir)
        .map_err(|e| format!("Failed to create {install_dir:?}: {e}"))?;
    let final_path = binary_path(install_dir);
    let tmp_path = install_dir.join(".gql.tmp");
    std::fs::write(&tmp_path, bytes)
        .map_err(|e| format!("Failed to write {tmp_path:?}: {e}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            &tmp_path,
            std::fs::Permissions::from_mode(0o755),
        )
        .map_err(|e| format!("Failed to chmod {tmp_path:?}: {e}"))?;
    }
    std::fs::rename(&tmp_path, &final_path)
        .map_err(|e| format!("Failed to move {tmp_path:?}: {e}"))?;
    Ok(())
}

fn print_path_hint(install_dir: &std::path::Path) {
    let install_dir_str = install_dir.to_string_lossy();
    let in_path = std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).any(|p| p == install_dir))
        .unwrap_or(false);
    if !in_path {
        println!(
            "\n{install_dir_str} is not on your PATH.\nAdd this to your \
             shell profile:\n\n    export PATH=\"{install_dir_str}:$PATH\"\n"
        );
    }
}

pub fn run(force: bool) -> Result<(), String> {
    let install_dir = install_dir()?;
    let current_version = installed_version(&install_dir);
    let asset_name = platform::asset_name()?;

    let client = reqwest::blocking::Client::new();
    let release = github::fetch_latest_release(&client)?;

    if !force && current_version.as_deref() == Some(release.tag_name.as_str()) {
        println!(
            "gql {} is already installed and up to date.",
            release.tag_name
        );
        print_path_hint(&install_dir);
        return Ok(());
    }

    let asset = release
        .assets
        .iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| {
            format!(
                "No asset named '{asset_name}' found in release {}",
                release.tag_name
            )
        })?;

    println!("Downloading gql {} ({asset_name})...", release.tag_name);
    let bytes = github::download_asset(&client, asset)?;
    write_binary(&install_dir, &bytes)?;
    std::fs::write(version_file(&install_dir), &release.tag_name)
        .map_err(|e| format!("Failed to write version file: {e}"))?;

    match current_version {
        Some(v) if v != release.tag_name => {
            println!("Updated gql {v} -> {}", release.tag_name)
        }
        Some(_) => println!("Reinstalled gql {}", release.tag_name),
        None => println!("Installed gql {}", release.tag_name),
    }
    print_path_hint(&install_dir);
    Ok(())
}
