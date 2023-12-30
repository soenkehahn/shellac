use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use std::os::unix::process::CommandExt;
use std::process::Command;
use swayipc::Connection;

type R<A> = Result<A, Box<dyn Error>>;

fn main() -> R<()> {
    let active_workspace = find_active_workspace()?;
    println!("active workspace: {active_workspace}");
    let config = read_config()?;
    println!("config: {config:?}");
    open_terminal(
        "alacritty",
        config
            .workspaces
            .get(&active_workspace)
            .map(|x| x.as_ref())
            .unwrap_or("zsh"),
    )?;
    Ok(())
}

fn find_active_workspace() -> R<String> {
    let mut connection = Connection::new()?;
    let workspaces = connection.get_workspaces()?;
    let active_workspace = workspaces
        .iter()
        .find(|workspace| workspace.focused)
        .ok_or("no focused workspace found")?;
    Ok(active_workspace.name.clone())
}

#[derive(Debug, Deserialize, Default)]
struct Config {
    workspaces: BTreeMap<String, String>,
}

fn read_config() -> R<Config> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("shellac")?;
    let config_file = xdg_dirs.place_config_file("config.yaml")?;
    let reader = match fs::File::open(config_file) {
        Ok(r) => r,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Config::default()),
        Err(e) => return Err(e)?,
    };
    Ok(serde_yaml::from_reader(reader)?)
}

struct Never(Box<Never>);

fn open_terminal(terminal: &str, command: &str) -> R<Never> {
    let mut args: Vec<&str> = Vec::new();
    args.push("--command");
    args.append(&mut command.split_whitespace().collect());
    Err(Command::new(terminal).args(args).exec())?
}
