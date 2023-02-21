fn get_shell_aliases() -> String {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| panic!("Unable to get shell name"));
    let bash_shell_aliases = r#"
pg() {
    cd $(p g $1)
}
px() {
    p  x "$@"
}
pl() {
    p ls
}
pc() {
    cd ~/.p/
}
pi() {
    p info
}
    "#;
    let zsh_shell_aliases = r#"
pg() {
    cd $(p g $1)
}
px() {
    p x "$@"
}
pl() {
    p ls
}
pc() {
    cd ~/.p/
}
pi() {
    p info
}
    "#;

    let mut shell_aliases = String::new();

    if shell.contains("bash") {
        shell_aliases.push_str(bash_shell_aliases);
    } else if shell.contains("zsh") {
        shell_aliases.push_str(zsh_shell_aliases);
    } else {
        println!("Your shell is not supported. Use the following aliases at your own risk:");
        shell_aliases.push_str(bash_shell_aliases);
    }

    shell_aliases.to_string()
}

pub fn log_shell_aliases() {
    println!("p offers some useful shell aliases:");
    println!("{}", get_shell_aliases());
    println!("You can add them to your shell configuration file. (e.g. ~/.bashrc, ~/.zshrc, ...)");
}