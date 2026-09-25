mod hyprland;
mod icon;
mod state;
mod utils;

fn get_env_var(key: &str) -> std::io::Result<String> {
    match std::env::var(key) {
        Ok(var) => Ok(var),
        Err(e) => Err(std::io::Error::other(format!("{key} env var unavailable: {e}"))),
    }
}

fn main() -> std::io::Result<()> {
    /* Compute paths */
    let home = get_env_var("HOME")?;
    let data_path = format!("{home}/.config/hyprway-bridge");
    let persistent_state_path = format!("{data_path}/persistent.json");
    let rendered_state_path = format!("{data_path}/rendered.json");

    /* Wait for the waybar to be ready */
    wait_for_waybar(std::time::Duration::from_secs(10))?;

    /* Init */
    let _ = init(&data_path, &persistent_state_path, &rendered_state_path);

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
                let _ = handle_event(&persistent_state_path, &rendered_state_path, &line);
            }
            Err(_) => { /* shhhh */ }
        }
    }

    Ok(())
}

fn init(data_path: &str, persistent_state_path: &str, rendered_state_path: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(data_path)?;
    let persistent_state = state::PersistentState::load_from_clients()?;
    persistent_state.save(&persistent_state_path)?;
    persistent_state.save_render(&rendered_state_path)?;
    Ok(())
}

fn handle_event(persistent_state_path: &str, rendered_state_path: &str, event: &str) -> std::io::Result<()> {
    let Some(event_kind) = event.split(">>").next() else {
        return Ok(());
    };
    match event_kind {
        "activewindow" | "workspace" => {
            /* Let's brute force it for now ? */
            let persistent_state = state::PersistentState::load_from_clients()?;
            persistent_state.save(persistent_state_path)?;
            persistent_state.save_render(rendered_state_path)?;
        }
        _ => {}
    }

    Ok(())
}

/// Blocks until every running waybar has installed its SIGRTMIN+8 handler.
fn wait_for_waybar(timeout: std::time::Duration) -> std::io::Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        // treat a lookup error (e.g. pidof finding nothing) as "not ready yet"
        let pids = utils::waybar_pids()?;
        let mut pids = pids.peekable();
        if pids.peek().is_some() && pids.all(|pid| catches_signal(pid, 42)) {
            return Ok(());
        }
        if std::time::Instant::now() >= deadline {
            return Err(std::io::Error::other("waybar did not become ready in time"));
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

/// True once the process has installed a handler for `signal`.
///
/// Super ugly code that goes read a file, but it's inly ran once at startup
fn catches_signal(pid: usize, signal: u32) -> bool {
    let Ok(status) = std::fs::read_to_string(format!("/proc/{pid}/status")) else {
        return false;
    };
    status
        .lines()
        .find_map(|line| line.strip_prefix("SigCgt:"))
        .and_then(|mask| u64::from_str_radix(mask.trim(), 16).ok())
        .is_some_and(|mask| mask & (1 << (signal - 1)) != 0)
}
