mod hyprland;
mod icon;
mod state;

fn get_env_var(key: &str) -> std::io::Result<String> {
    match std::env::var(key) {
        Ok(var) => Ok(var),
        Err(e) => Err(std::io::Error::other(format!("{key} env var unavailable: {e}"))),
    }
}

fn main() -> std::io::Result<()> {
    /* Init */
    let _ = init();

    /* Main loop: listen to the socket and update accordingly */
    use std::io::{BufRead, BufReader};
    use std::os::unix::net::UnixStream;

    let xdg_runtime_dir = get_env_var("XDG_RUNTIME_DIR")?;
    let hyprland_instance_sig = get_env_var("HYPRLAND_INSTANCE_SIGNATURE")?;

    let socket_path = format!("{xdg_runtime_dir}/hypr/{hyprland_instance_sig}/.socket2.sock");
    let stream = UnixStream::connect(socket_path)?;
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let _ = handle_event(&line);
            }
            Err(_) => { /* shhhh */ }
        }
    }

    Ok(())
}

fn init() -> std::io::Result<()> {
    let persistent_state = state::PersistentState::load_from_clients()?;
    persistent_state.render()?;
    Ok(())
}

fn handle_event(event: &str) -> std::io::Result<()> {
    let Some(event_kind) = event.split(">>").next() else {
        return Ok(());
    };
    match event_kind {
        "activewindow" | "workspace" => {
            /* Let's brute force it for now ? */
            let persistent_state = state::PersistentState::load_from_clients()?;
            persistent_state.render()?;
        }
        _ => {}
    }

    Ok(())
}
