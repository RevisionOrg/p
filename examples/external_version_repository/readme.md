# External Version Repository

A version repository contains a set of version configurations. These configurations can be used to add support for new project types to `p`. Treat this folder as if it is its own git repository. You can add it as a remote repository to your `p` configuration.

## How to use

### CLI

To add this repository to your `p` configuration, run the following command:
```bash
p repo add GIT_URL
```

Sync all of your repositories with the remote repositories:
```bash
p repo sync
```
This will download all of the version configurations from the remote repositories and store them in `~/.p/external_versions/`.

To list all of your repositories, run the following command:
```bash
p repo list
```

To remove this repository from your `p` configuration, run the following command:
```bash
p repo remove GIT_URL
```

### Configuration

If you don't want to use the CLI, add/remove the following line to your `~/.p/config.toml` file:
```toml
version_repositories = ["GIT_URL"]
```
