# p
p is a simple project management tool for the command line written in Rust

## Demo
![p demo](./examples/demo/demo.gif)

## Availability
p is available for macOS and Linux. If you're using Windows, use WSL.

## Installation
Download the binary from the latest release and install it.

## Configuration
All configuration is done in the ~/.p/ directory, which gets created when you run p for the first time. The central configuration file is ~/.p/config.toml.

## User Configuration
The following is the schema for the TOML configuration file, located at ~/.p/config.toml.

```Rust
pub struct UserConfigSchema {
    pub projects_dir: String,
    pub project_management_tool: String,
    pub version_repositories: Option<Vec<String>>,
    pub editor: Option<String>,
}
```

`projects_dir`: The directory where all your projects are stored.<br/>
`project_management_tool`: The default project management tool used by p.<br/>
`version_repositories`: Optional. A list of external version repositories.<br/>
`editor`: Optional. The default text editor used by p.<br/>

## Version Configuration
p treats each project as if it has its own 'version'. Versions are configurations that specify what kind of project it is and how to handle it. Versions are stored in ~/.p/versions/. To create a new version, create a new file in ~/.p/versions/. The file should contain a valid TOML configuration. The following is the TOML configuration schema for a version:

```Rust
pub struct VersionConfigSchema {
    pub version: String,
    pub description: String,
    pub files_needed: Vec<String>,
    pub directories_needed: Vec<String>,
    pub specificity: u8,
    pub project_management_tool: Option<String>,
}
```

`version`: The name of the version.<br/>
`description`: A short description of the version.<br/>
`files_needed`: A list of files that the version needs.<br/>
`directories_needed`: A list of directories.<br/>
`specificity`: The specificity of the version. The higher the number, the more specific the version is. For example, if you have a version for a Rust project and a version for a Rust project with a Cargo.toml file, the version with the Cargo.toml file should have a higher specificity.<br/>
`project_management_tool`: Optional. The project management tool used by p for this version.<br/>

## External Version Repositories
p supports external version repositories.

### Adding an External Version Repository
To add an external version repository, add the URL to the version_repositories list in ~/.p/config.toml by running `p repo add URL`.

### Syncing External Version Repositories
To sync external version repositories, run `p repo sync`. This will download all the versions from the external version repositories and store them in ~/.p/external_versions/.

### Removing an External Version Repository
To remove an external version repository, run `p repo remove URL`. This will remove the URL from the version_repositories list in ~/.p/config.toml.

### Creating a Version Repository
p provides an easy way to create a version repository. To create a version repository, run `p repo new NAME`. This will create a new directory in your current directory. The directory will contain a `versions` directory. You can add versions to the `versions` directory, initialize a new git repository and push the repository to GitHub. Other users can then add your version repository to their p configuration.