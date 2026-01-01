pub const ICONS: &[(&'static str, char)] = &[
    ("firefox", ''),
    ("dev.zed.Zed", ''),
    ("code", ''),
    ("kitty", ''),
    ("discord", '󰍩'),
    ("obsidian", '󰏫'),
    ("builtins.empty", EMPTY_ICON),
];
pub const EMPTY_ICON: char = '\u{2003}';
pub const UNKNWON_ICON: char = '';

pub fn get_icon(name: &str) -> char {
    ICONS
        .iter()
        .find(|(icon_name, _)| *icon_name == name)
        .map(|(_, icon)| *icon)
        .unwrap_or(UNKNWON_ICON)
}
