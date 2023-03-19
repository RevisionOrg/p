use self_update::{cargo_crate_version, get_target,};

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
        .build()?
        .update()?;

    if status.updated() {
        println!("Updated to version {}!", status.version());
    } else {
        println!("Already up to date");
    }

    Ok(())
}
