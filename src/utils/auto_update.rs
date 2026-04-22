use sha2::{Digest, Sha256};
use std::io::{Read, Write};

const MAX_DOWNLOAD_BYTES: u64 = 100 * 1024 * 1024;

pub fn get_platform_asset_name() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", "x86_64") => "todo-tui-windows-amd64.exe",
        ("linux", "x86_64") => "todo-tui-linux-amd64",
        ("macos", "aarch64") => "todo-tui-macos-arm64",
        ("macos", "x86_64") => "todo-tui-macos-amd64",
        _ => "",
    }
}

pub fn perform_update(version: &str) -> Result<String, String> {
    let asset = get_platform_asset_name();
    if asset.is_empty() {
        return Err("unsupported_platform".into());
    }

    let url = format!(
        "https://github.com/pfTheTrial/todo-tui/releases/download/v{}/{}",
        version, asset
    );

    let current_exe = std::env::current_exe().map_err(|e| format!("exe lookup: {}", e))?;

    let tmp_path = current_exe.with_extension("new");
    let checksum = download_checksum(version, asset)?;

    let agent = ureq::Agent::new_with_defaults();
    let mut response = agent
        .get(&url)
        .header("User-Agent", "tdt-auto-updater")
        .call()
        .map_err(|e| format!("download: {}", e))?;

    let actual_checksum = write_response_with_hash(&mut response, &tmp_path)?;
    if actual_checksum != checksum {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(format!(
            "checksum mismatch: expected {}, got {}",
            checksum, actual_checksum
        ));
    }

    // Platform-specific replacement
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("chmod: {}", e))?;
        std::fs::rename(&tmp_path, &current_exe).map_err(|e| format!("rename: {}", e))?;
    }

    #[cfg(windows)]
    {
        let old_path = current_exe.with_extension("old");
        let _ = std::fs::remove_file(&old_path); // Clean up previous .old
        std::fs::rename(&current_exe, &old_path).map_err(|e| format!("rename current: {}", e))?;
        std::fs::rename(&tmp_path, &current_exe).map_err(|e| format!("rename new: {}", e))?;
    }

    Ok(version.to_string())
}

fn download_checksum(version: &str, asset: &str) -> Result<String, String> {
    let url = format!(
        "https://github.com/pfTheTrial/todo-tui/releases/download/v{}/{}.sha256",
        version, asset
    );
    let agent = ureq::Agent::new_with_defaults();
    let mut response = agent
        .get(&url)
        .header("User-Agent", "tdt-auto-updater")
        .call()
        .map_err(|e| format!("checksum download: {}", e))?;
    let mut body = String::new();
    response
        .body_mut()
        .as_reader()
        .read_to_string(&mut body)
        .map_err(|e| format!("checksum read: {}", e))?;
    parse_checksum(&body).ok_or_else(|| "checksum file did not contain a SHA256 digest".to_string())
}

fn parse_checksum(body: &str) -> Option<String> {
    body.split_whitespace()
        .find(|part| part.len() == 64 && part.chars().all(|c| c.is_ascii_hexdigit()))
        .map(|part| part.to_ascii_lowercase())
}

fn write_response_with_hash(
    response: &mut ureq::http::Response<ureq::Body>,
    path: &std::path::Path,
) -> Result<String, String> {
    let mut reader = response.body_mut().as_reader();
    let mut file = std::fs::File::create(path).map_err(|e| format!("write: {}", e))?;
    let mut hasher = Sha256::new();
    let mut total = 0u64;
    let mut buffer = [0u8; 16 * 1024];

    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(|e| format!("read: {}", e))?;
        if read == 0 {
            break;
        }
        total += read as u64;
        if total > MAX_DOWNLOAD_BYTES {
            let _ = std::fs::remove_file(path);
            return Err("download exceeded maximum expected size".to_string());
        }
        hasher.update(&buffer[..read]);
        file.write_all(&buffer[..read])
            .map_err(|e| format!("write: {}", e))?;
    }
    file.flush().map_err(|e| format!("flush: {}", e))?;
    Ok(hex::encode(hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::parse_checksum;

    #[test]
    fn parses_sha256_from_checksum_file() {
        let digest = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        assert_eq!(
            parse_checksum(&format!("{}  todo-tui-linux-amd64", digest)),
            Some(digest.to_string())
        );
    }

    #[test]
    fn rejects_invalid_checksum_file() {
        assert_eq!(parse_checksum("not-a-checksum"), None);
    }
}
