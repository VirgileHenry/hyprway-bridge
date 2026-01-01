const WORKSPACE_COUNT: usize = 10;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WorkspaceState {
    active: bool,
    icon: char,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PersistentState {
    workspaces: [WorkspaceState; WORKSPACE_COUNT],
}

impl PersistentState {
    pub fn load_from_clients() -> std::io::Result<Self> {
        let clients = crate::hyprland::load_clients()?;

        /* Keep track of sizes, master window is the largest one */
        let mut workspaces: [(WorkspaceState, usize); WORKSPACE_COUNT] = std::array::from_fn(|_| {
            (
                WorkspaceState {
                    active: false,
                    icon: crate::icon::EMPTY_ICON,
                },
                usize::MAX,
            )
        });

        for client in clients.iter() {
            let index = client.workspace.id - 1;
            if client.focus_history_id < workspaces[index].1 {
                workspaces[index].0.icon = crate::icon::get_icon(&client.class_name);
                workspaces[index].1 = client.focus_history_id;
            }
        }

        let active_workspace = crate::hyprland::load_active_workspace()?;
        workspaces[active_workspace.id - 1].0.active = true;

        Ok(Self {
            workspaces: std::array::from_fn(|i| workspaces[i].0.clone()),
        })
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        use std::io::Error;

        let json = serde_json::to_string(self).map_err(|e| Error::other(format!("Failed to serialize to json: {e}")))?;
        std::fs::write(path, json)?;

        Ok(())
    }

    pub fn save_render(&self, path: &str) -> std::io::Result<()> {
        use std::io::Error;
        let mut text = format!("[");

        for workspace in self.workspaces.iter() {
            let icon = workspace.icon;
            let color = if workspace.active { "orange" } else { "white" };
            text.push_str(&format!("<span foreground='{color}'> {icon} </span>"));
        }
        text.push_str("]");

        let rendered = RenderedState { text };
        let json = serde_json::to_string(&rendered).map_err(|e| Error::other(format!("Failed to serialize to json: {e}")))?;
        std::fs::write(path, json)?;

        /* Signal waybar an update was made */
        let output = std::process::Command::new("pidof").arg("waybar").output()?;
        let waybar_pids = String::from_utf8(output.stdout).map_err(|e| Error::other(format!("invalid utf8 from pidof: {e}")))?;

        for waybar_pid in waybar_pids.split(' ') {
            let status = std::process::Command::new("kill")
                .arg("-s")
                .arg("RTMIN+8")
                .arg(waybar_pid.trim())
                .status()?;

            if !status.success() {
                return Err(Error::other(format!("Failed to send signal to waybar")));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize)]
pub struct RenderedState {
    text: String,
}
