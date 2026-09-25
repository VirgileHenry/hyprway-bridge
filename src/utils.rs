/// Get the waybar pids using the `pidof` command.
pub fn waybar_pids() -> std::io::Result<impl Iterator<Item = usize>> {
    use std::io::Error;

    let output = std::process::Command::new("pidof").arg("waybar").output()?;
    let waybar_pids = String::from_utf8(output.stdout).map_err(|e| Error::other(format!("invalid utf8 from pidof: {e}")))?;
    let result = waybar_pids
        .split(' ')
        .map(|pid| usize::from_str_radix(pid.trim(), 10).map_err(|e| Error::other(format!("Invalid usize for pid: {e}"))))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(result.into_iter())
}
