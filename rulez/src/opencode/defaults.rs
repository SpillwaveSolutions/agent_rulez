use std::path::PathBuf;

pub fn default_audit_log_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not determine home directory");
    path.push(".config");
    path.push("opencode");
    path.push("plugins");
    path.push("rulez-plugin");
    path.push("audit.log");
    path
}

pub fn default_rulez_binary_path() -> String {
    "rulez".to_string()
}

pub fn default_event_filters() -> Vec<String> {
    vec![]
}
