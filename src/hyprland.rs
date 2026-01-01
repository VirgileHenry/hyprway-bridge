#[allow(unused)]
#[derive(Debug, serde::Deserialize)]
pub struct Client {
    pub address: String,
    pub mapped: bool,
    pub hidden: bool,
    pub at: [usize; 2],
    pub size: [usize; 2],
    pub workspace: Workspace,
    pub floating: bool,
    pub pseudo: bool,
    pub monitor: usize,
    #[serde(rename = "class")]
    pub class_name: String,
    pub title: String,
    #[serde(rename = "initialClass")]
    pub initial_class: String,
    #[serde(rename = "initialTitle")]
    pub initial_title: String,
    pub pid: usize,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: usize,
    #[serde(rename = "fullscreenClient")]
    pub fullscreen_client: usize,
    pub grouped: Vec<String>,
    pub tags: Vec<String>,
    pub swallowing: String,
    #[serde(rename = "focusHistoryID")]
    pub focus_history_id: usize,
    #[serde(rename = "inhibitingIdle")]
    pub inhibiting_idle: bool,
    #[serde(rename = "xdgTag")]
    pub xdg_tag: String,
    #[serde(rename = "xdgDescription")]
    pub xdg_description: String,
}

#[allow(unused)]
#[derive(Debug, serde::Deserialize)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
}

pub fn load_clients() -> std::io::Result<Vec<Client>> {
    use std::io::{Error, ErrorKind};

    let output = std::process::Command::new("hyprctl").args(["clients", "-j"]).output()?;

    if !output.status.success() {
        return Err(Error::new(ErrorKind::Other, "hyprctl clients -j failed"));
    }

    let clients: Vec<Client> = serde_json::from_slice(&output.stdout).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok(clients)
}

#[allow(unused)]
#[derive(Debug, serde::Deserialize)]
pub struct ActiveWorkspace {
    pub id: usize,
    pub name: String,
    pub monitor: String,
    pub windows: usize,
    pub hasfullscreen: bool,
    pub lastwindow: String,
    pub lastwindowtitle: String,
}

pub fn load_active_workspace() -> std::io::Result<ActiveWorkspace> {
    use std::io::{Error, ErrorKind};

    let output = std::process::Command::new("hyprctl")
        .args(["activeworkspace", "-j"])
        .output()?;

    if !output.status.success() {
        return Err(Error::new(ErrorKind::Other, "hyprctl activeworkspace failed"));
    }

    let ws: ActiveWorkspace = serde_json::from_slice(&output.stdout).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok(ws)
}
