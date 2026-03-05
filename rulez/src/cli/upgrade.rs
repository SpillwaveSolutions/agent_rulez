//! `rulez upgrade` — check for and install newer binary releases from GitHub.

use anyhow::Result;

/// Run the upgrade command.
///
/// With `--check`: prints current and latest version, exits 0 if current, exits 1 if update available.
/// Without `--check`: downloads and installs the latest binary if a newer version exists.
pub async fn run(check_only: bool) -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");

    println!("Current version: {}", current_version);
    println!("Checking GitHub releases for latest version...");

    // Build the updater
    // NOTE: Replace "SpillwaveSolutions" and "agent_rulez" with actual GitHub owner/repo
    // when the public repo is configured. These values are placeholders.
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("SpillwaveSolutions")
        .repo_name("agent_rulez")
        .build()?
        .fetch()?;

    if releases.is_empty() {
        println!("No releases found on GitHub. Is the repository public with releases?");
        return Ok(());
    }

    let latest = &releases[0];
    let latest_version = latest.version.trim_start_matches('v');

    println!("Latest version: {}", latest_version);

    // Compare versions using self_update's built-in comparison
    if !self_update::version::bump_is_greater(current_version, latest_version).unwrap_or(false) {
        println!("Already on the latest version.");
        return Ok(());
    }

    println!(
        "Upgrade available: {} -> {}",
        current_version, latest_version
    );

    if check_only {
        println!("Run 'rulez upgrade' (without --check) to install.");
        return Ok(());
    }

    println!("Downloading and installing {}...", latest_version);

    let status = self_update::backends::github::Update::configure()
        .repo_owner("SpillwaveSolutions")
        .repo_name("agent_rulez")
        .bin_name("rulez")
        .current_version(current_version)
        .build()?
        .update()?;

    match status {
        self_update::Status::UpToDate(v) => {
            println!("Already up to date: {}", v);
        }
        self_update::Status::Updated(v) => {
            println!("Successfully upgraded to {}!", v);
            println!("Restart rulez to use the new version.");
        }
    }

    Ok(())
}
