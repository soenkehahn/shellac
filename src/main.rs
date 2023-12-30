use std::error::Error;
use swayipc::Connection;

type R<A> = Result<A, Box<dyn Error>>;

fn main() -> R<()> {
    let workspace = find_active_workspace()?;
    println!("active workspace: {workspace}");
    Ok(())
}

fn find_active_workspace() -> R<String> {
    let mut connection = Connection::new()?;
    let workspaces = connection.get_workspaces()?;
    let active_workspace = workspaces
        .iter()
        .find(|workspace| workspace.focused)
        .ok_or("no focused workspace found")?;
    return Ok(active_workspace.name.clone());
}
