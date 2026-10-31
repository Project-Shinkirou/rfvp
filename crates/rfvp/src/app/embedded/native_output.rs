//! Conversion metadata affects save isolation only, never HCB decoding or VM values.
use anyhow::{ensure, Context, Result};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{io::Read, path::{Path, PathBuf}};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Identity {
    version: u32,
    sha256: String,
}

pub(super) fn game_file(root: &Path, relative: &str) -> Result<PathBuf> {
    let path = Path::new(relative);
    ensure!(relative.len() <= 4096 && !path.is_absolute() && !relative.contains([':', '\\', '\0'])
        && path.components().all(|part| matches!(part, std::path::Component::Normal(_))),
        "RFVP_GAME_FILE_PATH_INVALID");
    let root = root.canonicalize()?;
    let path = root.join(path).canonicalize()?;
    ensure!(path.starts_with(&root), "RFVP_GAME_FILE_OUTSIDE_ROOT");
    Ok(path)
}

pub(super) fn save_namespace(root: &Path, script: &[u8]) -> Result<Option<String>> {
    let marker = root.join("shinki-native-script.json");
    match std::fs::symlink_metadata(&marker) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
        Ok(_) => {}
    }
    let path = game_file(root, "shinki-native-script.json")?;
    let file = std::fs::File::open(path)?;
    ensure!(file.metadata()?.len() <= 4096, "RFVP_NATIVE_IDENTITY_TOO_LARGE");
    let mut bytes = Vec::new();
    file.take(4097).read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 4096, "RFVP_NATIVE_IDENTITY_TOO_LARGE");
    let identity: Identity = serde_json::from_slice(&bytes).context("RFVP_NATIVE_IDENTITY_INVALID")?;
    ensure!(identity.version == 1 && identity.sha256.len() == 64
        && identity.sha256.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "RFVP_NATIVE_IDENTITY_INVALID");
    let actual = format!("{:x}", Sha256::digest(script));
    ensure!(actual == identity.sha256, "RFVP_NATIVE_IDENTITY_SCRIPT_MISMATCH");
    Ok(Some(format!(".shinki/rfvp/{actual}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conversion_marker_is_explicit_hash_bound_and_fail_closed() {
        let root = std::env::temp_dir().join(format!("rfvp-native-identity-{}", std::process::id()));
        std::fs::create_dir_all(root.join(".shinki")).unwrap();
        assert_eq!(save_namespace(&root, b"original").unwrap(), None);
        let marker = root.join("shinki-native-script.json");
        let hash = format!("{:x}", Sha256::digest(b"native"));
        std::fs::write(&marker, format!(r#"{{"version":1,"sha256":"{hash}"}}"#)).unwrap();
        assert_eq!(save_namespace(&root, b"native").unwrap(), Some(format!(".shinki/rfvp/{hash}")));
        assert!(save_namespace(&root, b"original").is_err());
        use crate::host_api::{RfvpStorage, SubdirStorage, std_fs::StdDir};
        let disk = std::sync::Arc::new(StdDir::new(&root));
        disk.write("save/rfvp_global.bin", b"old read history").unwrap();
        let scoped = SubdirStorage::new(disk.clone(), &save_namespace(&root, b"native").unwrap().unwrap());
        assert!(!scoped.exists("save/rfvp_global.bin"));
        scoped.write("save/rfvp_global.bin", b"native read history").unwrap();
        assert_eq!(scoped.read("save/rfvp_global.bin").unwrap(), b"native read history");
        assert_eq!(disk.read("save/rfvp_global.bin").unwrap(), b"old read history");
        for invalid in [r#"{"version":2,"sha256":"x"}"#, r#"{"version":1,"sha256":"x","text":[]}"#, "{"] {
            std::fs::write(&marker, invalid).unwrap();
            assert!(save_namespace(&root, b"native").is_err());
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}
