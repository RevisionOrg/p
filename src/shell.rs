use crate::{AliasArgs, Shell};

fn get_shell_aliases(alias_args: &AliasArgs) -> String {
    let bash_shell_aliases = r#"
pg() {
    cd $(p go $1)
}
px() {
    p  execute "$@"
}
pl() {
    p list
}
pc() {
    cd ~/.p/
}
pi() {
    p info
}
pf() {
    p find "$@"
}
pfg() {
    pg "$(p find -c -a 1 "$@")"
}
"#;
    let zsh_shell_aliases = r#"
pg() {
    cd $(p go $1)
}
px() {
    p execute "$@"
}
pl() {
    p list
}
pc() {
    cd ~/.p/
}
pi() {
    p info
}
pf() {
    p find "$@"
}
pfg() {
    pg "$(p find -c -a 1 "$@")"
}
    "#;

    let fish_shell_aliases = r#"
function pg
    cd (p go $argv[1])
end

function px
    p execute $argv
end

function pl
    p list
end

function pc
    cd ~/.p/
end

function pi
    p info
end

function pf
    p find $argv
end

function pfg
    pg (p find -c -a 1 $argv)
end
    "#;

    let powershell_shell_aliases = r#"
function pg {
    Set-Location (p go $args[0])
}

function px {
    p execute @args
}

function pl {
    p list
}

function pc {
    Set-Location ~/.p/
}

function pi {
    p info
}

function pf {
    p find @args
}

function pfg {
    pg (p find -c -a 1 @args)
}
    "#;

    let elvish_shell_aliases = r#"
fn pg {
    cd (p go $args[0])
}

fn px {
    p execute $args
}

fn pl {
    p list
}

fn pc {
    cd ~/.p/
}

fn pi {
    p info
}

fn pf {
    p find $args
}

fn pfg {
    pg (p find -c -a 1 $args)
}
    "#;

    match alias_args.shell {
        Shell::Bash => bash_shell_aliases.to_string(),
        Shell::Zsh => zsh_shell_aliases.to_string(),
        Shell::Fish => fish_shell_aliases.to_string(),
        Shell::Powershell => powershell_shell_aliases.to_string(),
        Shell::Elvish => elvish_shell_aliases.to_string(),
    }
}

pub fn log_shell_aliases(alias_args: &AliasArgs) {
    println!("p offers some useful shell aliases:");
    println!("{}", get_shell_aliases(&alias_args));
    println!("You can add them to your shell configuration file. (e.g. ~/.bashrc, ~/.zshrc, ...)");
}
