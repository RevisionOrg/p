use self_update::cargo_crate_version;

pub fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let owner = "Coyenn";
    let repo = "p";
    let status = self_update::backends::github::Update::configure()
        .repo_owner(owner)
        .repo_name(repo)
        .bin_name("p")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    println!("Update status: `{}`!", status.version());
    Ok(())
}