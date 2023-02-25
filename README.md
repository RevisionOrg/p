# p
A small tool to manage your projects.

## What can it do?

- List all of your projects (`p list`/`pl`)
- Navigate to a project (`p go`/`pg`)
- Execute a command in the current project (`p execute`/`px`)

(The short commands can only be used if you have added the aliases (`p aliases`) to your shell)

# Installation
Download the binary from the latest release and install it.

# Configuration
All configuration is done in the `~/.p/` directory. This directory gets created when you run `p` for the first time.
The central configuration file is `~/.p/config.toml`.
`p` treats each project as if it has its own 'version'. Versions are configurations that specify what kind of project it is and how to handle it. Versions are stored in `~/.p/versions/`. To create a new version, create a new file in `~/.p/versions/`. The file should contain a valid TOML configuration. A sample version is provided in `~/.p/versions/rust.toml`.