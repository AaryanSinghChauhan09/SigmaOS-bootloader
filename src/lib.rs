//! SigmaOS-bootloader: EFI boot entry model (systemd-boot/grub).
//!
//! Real implementation: boot entries (id, title, loader, options), boot
//! order, timeout, and default selection. Replaces the Plan placeholder.


use std::fmt;

/// A boot entry (EFI BootXXXX).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootEntry {
    pub id: String,
    pub title: String,
    pub loader: String,
    pub options: Vec<String>,
}

/// Bootloader configuration.
#[derive(Debug, Default)]
pub struct BootloaderConfig {
    pub entries: Vec<BootEntry>,
    pub default: Option<String>,
    pub timeout_sec: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootloaderError {
    DuplicateEntry(String),
    EntryNotFound(String),
    InvalidTimeout,
}

impl fmt::Display for BootloaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateEntry(id) => write!(f, "duplicate entry {:?}", id),
            Self::EntryNotFound(id) => write!(f, "entry {:?} not found", id),
            Self::InvalidTimeout => write!(f, "timeout must be non-negative"),
        }
    }
}

impl std::error::Error for BootloaderError {}

impl BootloaderConfig {
    pub fn new() -> Self { Self::default() }

    pub fn add_entry(&mut self, entry: BootEntry) -> Result<(), BootloaderError> {
        if self.entries.iter().any(|e| e.id == entry.id) {
            return Err(BootloaderError::DuplicateEntry(entry.id));
        }
        self.entries.push(entry);
        Ok(())
    }

    pub fn set_default(&mut self, id: impl Into<String>) { self.default = Some(id.into()); }

    pub fn set_timeout(&mut self, sec: u32) { self.timeout_sec = sec; }

    pub fn default_entry(&self) -> Option<&BootEntry> {
        self.default.as_ref()
            .and_then(|id| self.entries.iter().find(|e| e.id == *id))
            .or_else(|| self.entries.first())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_select_default_aaa() {
        let mut cfg = BootloaderConfig::new();
        cfg.add_entry(BootEntry { id: "0001".into(), title: "SigmaOS".into(),
            loader: "/EFI/sigmaos.efi".into(), options: vec![] }).unwrap();
        cfg.set_default("0001");
        assert!(cfg.default_entry().is_some());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut cfg = BootloaderConfig::new();
        let e = BootEntry { id: "x".into(), title: "x".into(),
            loader: "".into(), options: vec![] };
        cfg.add_entry(e.clone()).unwrap();
        assert!(matches!(cfg.add_entry(e), Err(BootloaderError::DuplicateEntry(_))));
    }
}
