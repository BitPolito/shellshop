//! Load or generate a persistent Ed25519 SSH host key.

use std::fs;
use std::path::Path;

use anyhow::Context;
use russh::keys::ssh_key::{LineEnding, rand_core::OsRng};
use russh::keys::{Algorithm, PrivateKey};

/// Load an OpenSSH-format private key from `path`, or generate Ed25519 and write it on first run.
pub fn load_or_generate(path: &Path) -> anyhow::Result<PrivateKey> {
    if path.exists() {
        let pem = fs::read_to_string(path)
            .with_context(|| format!("read host key {}", path.display()))?;
        PrivateKey::from_openssh(pem.trim()).map_err(|e| anyhow::anyhow!(e))
    } else {
        let key = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)
            .map_err(|e| anyhow::anyhow!("generate Ed25519 host key: {e}"))?;
        let pem = key
            .to_openssh(LineEnding::LF)
            .map_err(|e| anyhow::anyhow!("encode host key: {e}"))?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create directory {}", parent.display()))?;
        }
        fs::write(path, pem.as_bytes())
            .with_context(|| format!("write host key {}", path.display()))?;
        Ok(key)
    }
}
