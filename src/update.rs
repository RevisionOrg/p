use log::error;
use self_update::{cargo_crate_version, get_target};

pub fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let owner = "Coyenn";
    let repo = "p";
    let target = get_target();
    let identifier = format!("p-{}.tar.gz", target);

    let status = self_update::backends::github::Update::configure()
        .repo_owner(owner)
        .repo_name(repo)
        .identifier(&identifier)
        .bin_name("p")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build().unwrap_or_else(|e| {
            error!("{}", e);
            std::process::exit(1);
        })
        .update().unwrap_or_else(|e| {
            // If it is a permission denied error, suggest to run as sudo
            if let self_update::errors::Error::Io(e) = &e {
                if e.kind() == ::std::io::ErrorKind::PermissionDenied {
                    error!("Permission denied while installing update. Try running the command as sudo.");
                    std::process::exit(1);
                }
            } else {
                error!("{}", e);
            }

            std::process::exit(1);
        });

    if status.updated() {
        println!("Updated to version {}!", status.version());
        println!(
            "Don't forget to update your shell aliases and shell completions, if you use them."
        );
    } else {
        println!("Already up to date");
    }

    Ok(())
}
