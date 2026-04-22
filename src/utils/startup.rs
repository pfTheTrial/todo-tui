pub fn set_startup_with_os(enabled: bool) -> Result<(), String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("Não foi possível localizar o executável: {e}"))?;
    let spec = startup_spec(&exe)?;

    if enabled {
        if let Some(parent) = spec.path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("startup mkdir: {e}"))?;
        }
        std::fs::write(&spec.path, spec.content).map_err(|e| format!("startup write: {e}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&spec.path, std::fs::Permissions::from_mode(0o755));
        }
        Ok(())
    } else {
        if spec.path.exists() {
            std::fs::remove_file(&spec.path).map_err(|e| format!("startup remove: {e}"))?;
        }
        Ok(())
    }
}

struct StartupSpec {
    path: std::path::PathBuf,
    content: String,
}

fn startup_spec(exe: &std::path::Path) -> Result<StartupSpec, String> {
    let exe = exe
        .canonicalize()
        .unwrap_or_else(|_| exe.to_path_buf())
        .to_string_lossy()
        .to_string();
    startup_spec_with_home(&exe, dirs::home_dir(), dirs::config_dir())
}

fn startup_spec_with_home(
    exe: &str,
    home_dir: Option<std::path::PathBuf>,
    config_dir: Option<std::path::PathBuf>,
) -> Result<StartupSpec, String> {
    match std::env::consts::OS {
        "windows" => {
            let home = home_dir.ok_or("Diretório HOME não encontrado para configurar startup.")?;
            let path = home
                .join("AppData")
                .join("Roaming")
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("Startup")
                .join("tdt.cmd");
            Ok(StartupSpec {
                path,
                content: format!("@echo off\r\nstart \"tdt\" \"{}\"\r\n", exe),
            })
        }
        "linux" => {
            let config =
                config_dir.ok_or("Diretório de configuração não encontrado para autostart.")?;
            let path = config.join("autostart").join("tdt.desktop");
            Ok(StartupSpec {
                path,
                content: format!(
                    "[Desktop Entry]\nType=Application\nName=tdt\nExec={}\nTerminal=true\nX-GNOME-Autostart-enabled=true\n",
                    shell_escape(exe)
                ),
            })
        }
        "macos" => {
            let home = home_dir.ok_or("Diretório HOME não encontrado para LaunchAgents.")?;
            let path = home
                .join("Library")
                .join("LaunchAgents")
                .join("com.todo-tui.tdt.plist");
            Ok(StartupSpec {
                path,
                content: format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
<plist version=\"1.0\"><dict><key>Label</key><string>com.todo-tui.tdt</string>\
<key>ProgramArguments</key><array><string>{}</string></array>\
<key>RunAtLoad</key><true/></dict></plist>\n",
                    xml_escape(exe)
                ),
            })
        }
        other => Err(format!("Startup automático não suportado para {other}.")),
    }
}

fn shell_escape(value: &str) -> String {
    if value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "/._-".contains(c))
    {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::{shell_escape, startup_spec_with_home, xml_escape};

    #[test]
    fn builds_startup_spec_for_current_platform() {
        let dir = tempfile::tempdir().unwrap();
        let spec = startup_spec_with_home(
            "C:/Program Files/tdt/tdt.exe",
            Some(dir.path().to_path_buf()),
            Some(dir.path().join(".config")),
        )
        .unwrap();

        assert!(spec.path.to_string_lossy().contains("tdt"));
        assert!(spec.content.contains("tdt"));
    }

    #[test]
    fn escapes_shell_paths_with_spaces() {
        assert_eq!(shell_escape("/tmp/tdt"), "/tmp/tdt");
        assert_eq!(shell_escape("/tmp/my app/tdt"), "'/tmp/my app/tdt'");
    }

    #[test]
    fn escapes_xml_content() {
        assert_eq!(xml_escape("a&b<c>"), "a&amp;b&lt;c&gt;");
    }
}
