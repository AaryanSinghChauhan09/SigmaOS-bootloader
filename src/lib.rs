//! SigmaOS-bootloader: Unified bootloader configuration with OOPS principles
//!
//! OOPS Principles Applied:
//! - Abstraction: Bootloader trait provides abstraction for different bootloaders
//! - Polymorphism: Different bootloaders used interchangeably
//! - Encapsulation: Internal implementation details hidden
//! - Interface Segregation: Small, focused trait methods
//! - Dependency Inversion: Depend on Bootloader trait, not concrete implementations
//!
//! Design Patterns:
//! - Strategy Pattern: Different bootloader strategies
//! - Factory Pattern: BootloaderFactory creates appropriate bootloader
//! - Template Method: BootloaderManager defines workflow
//! - Command Pattern: Boot operations as commands

use std::fmt;
use std::error::Error;

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
    InstallFailed(String),
    UninstallFailed(String),
}

impl fmt::Display for BootloaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateEntry(id) => write!(f, "duplicate entry {:?}", id),
            Self::EntryNotFound(id) => write!(f, "entry {:?} not found", id),
            Self::InvalidTimeout => write!(f, "timeout must be non-negative"),
            Self::InstallFailed(msg) => write!(f, "install failed: {}", msg),
            Self::UninstallFailed(msg) => write!(f, "uninstall failed: {}", msg),
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

/// Bootloader trait - Abstraction for different bootloader implementations
/// This allows polymorphism - any bootloader can be used interchangeably
pub trait Bootloader {
    /// Install the bootloader
    fn install(&self) -> Result<(), Box<dyn Error>>;
    
    /// Uninstall the bootloader
    fn uninstall(&self) -> Result<(), Box<dyn Error>>;
    
    /// Add a kernel entry
    fn add_kernel_entry(&self, kernel_version: &str, kernel_path: &str, initrd_path: &str) -> Result<(), Box<dyn Error>>;
    
    /// Remove a kernel entry
    fn remove_kernel_entry(&self, kernel_version: &str) -> Result<(), Box<dyn Error>>;
    
    /// Set default boot entry
    fn set_default_entry(&self, entry_id: &str) -> Result<(), Box<dyn Error>>;
    
    /// List all boot entries
    fn list_entries(&self) -> Result<Vec<BootEntry>, Box<dyn Error>>;
    
    /// Enable Secure Boot
    fn enable_secure_boot(&self) -> Result<(), Box<dyn Error>>;
    
    /// Disable Secure Boot
    fn disable_secure_boot(&self) -> Result<(), Box<dyn Error>>;
}

/// Systemd-boot implementation
pub struct SystemdBoot {
    esp_path: String,
}

impl SystemdBoot {
    pub fn new(esp_path: &str) -> Self {
        Self {
            esp_path: esp_path.to_string(),
        }
    }
}

/// Implement Bootloader trait for Systemd-boot
/// This demonstrates polymorphism - SystemdBoot is a Bootloader
impl Bootloader for SystemdBoot {
    fn install(&self) -> Result<(), Box<dyn Error>> {
        println!("Installing systemd-boot to {}", self.esp_path);
        Ok(())
    }
    
    fn uninstall(&self) -> Result<(), Box<dyn Error>> {
        println!("Uninstalling systemd-boot from {}", self.esp_path);
        Ok(())
    }
    
    fn add_kernel_entry(&self, kernel_version: &str, kernel_path: &str, initrd_path: &str) -> Result<(), Box<dyn Error>> {
        println!("Adding kernel entry: {} -> {} with initrd {}", kernel_version, kernel_path, initrd_path);
        Ok(())
    }
    
    fn remove_kernel_entry(&self, kernel_version: &str) -> Result<(), Box<dyn Error>> {
        println!("Removing kernel entry: {}", kernel_version);
        Ok(())
    }
    
    fn set_default_entry(&self, entry_id: &str) -> Result<(), Box<dyn Error>> {
        println!("Setting default entry: {}", entry_id);
        Ok(())
    }
    
    fn list_entries(&self) -> Result<Vec<BootEntry>, Box<dyn Error>> {
        println!("Listing boot entries from {}", self.esp_path);
        Ok(vec![])
    }
    
    fn enable_secure_boot(&self) -> Result<(), Box<dyn Error>> {
        println!("Enabling Secure Boot for systemd-boot");
        Ok(())
    }
    
    fn disable_secure_boot(&self) -> Result<(), Box<dyn Error>> {
        println!("Disabling Secure Boot for systemd-boot");
        Ok(())
    }
}

/// GRUB implementation
pub struct Grub {
    install_device: String,
}

impl Grub {
    pub fn new(install_device: &str) -> Self {
        Self {
            install_device: install_device.to_string(),
        }
    }
}

/// Implement Bootloader trait for GRUB
/// This demonstrates polymorphism - GRUB is also a Bootloader
impl Bootloader for Grub {
    fn install(&self) -> Result<(), Box<dyn Error>> {
        println!("Installing GRUB to {}", self.install_device);
        Ok(())
    }
    
    fn uninstall(&self) -> Result<(), Box<dyn Error>> {
        println!("Uninstalling GRUB from {}", self.install_device);
        Ok(())
    }
    
    fn add_kernel_entry(&self, kernel_version: &str, _kernel_path: &str, _initrd_path: &str) -> Result<(), Box<dyn Error>> {
        println!("Adding GRUB menu entry for kernel: {}", kernel_version);
        Ok(())
    }
    
    fn remove_kernel_entry(&self, kernel_version: &str) -> Result<(), Box<dyn Error>> {
        println!("Removing GRUB menu entry for kernel: {}", kernel_version);
        Ok(())
    }
    
    fn set_default_entry(&self, entry_id: &str) -> Result<(), Box<dyn Error>> {
        println!("Setting GRUB default entry: {}", entry_id);
        Ok(())
    }
    
    fn list_entries(&self) -> Result<Vec<BootEntry>, Box<dyn Error>> {
        println!("Listing GRUB menu entries");
        Ok(vec![])
    }
    
    fn enable_secure_boot(&self) -> Result<(), Box<dyn Error>> {
        println!("Enabling Secure Boot for GRUB");
        Ok(())
    }
    
    fn disable_secure_boot(&self) -> Result<(), Box<dyn Error>> {
        println!("Disabling Secure Boot for GRUB");
        Ok(())
    }
}

/// Bootloader factory - Factory Pattern
/// Creates appropriate bootloader based on configuration
pub struct BootloaderFactory;

impl BootloaderFactory {
    pub fn create_bootloader(bootloader_type: &str, config: &str) -> Box<dyn Bootloader> {
        match bootloader_type {
            "systemd-boot" => Box::new(SystemdBoot::new(config)),
            "grub" => Box::new(Grub::new(config)),
            _ => panic!("Unknown bootloader type: {}", bootloader_type),
        }
    }
}

/// Bootloader manager - Template Method Pattern
/// Manages bootloader lifecycle with a predefined workflow
pub struct BootloaderManager {
    bootloader: Box<dyn Bootloader>,
}

impl BootloaderManager {
    pub fn new(bootloader: Box<dyn Bootloader>) -> Self {
        Self { bootloader }
    }
    
    /// Template method - defines the bootloader management workflow
    pub fn manage_bootloader(&self, action: &str) -> Result<(), Box<dyn Error>> {
        match action {
            "install" => self.bootloader.install(),
            "uninstall" => self.bootloader.uninstall(),
            "list" => {
                let entries = self.bootloader.list_entries()?;
                println!("Boot entries: {:?}", entries);
                Ok(())
            }
            _ => Err("Unknown action".into()),
        }
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
    
    #[test]
    fn test_systemd_boot_creation() {
        let systemd_boot = SystemdBoot::new("/boot/efi");
        assert_eq!(systemd_boot.esp_path, "/boot/efi");
    }
    
    #[test]
    fn test_grub_creation() {
        let grub = Grub::new("/dev/sda");
        assert_eq!(grub.install_device, "/dev/sda");
    }
    
    #[test]
    fn test_bootloader_factory() {
        let systemd_boot = BootloaderFactory::create_bootloader("systemd-boot", "/boot/efi");
        let grub = BootloaderFactory::create_bootloader("grub", "/dev/sda");
        
        // Both are Bootloader trait objects - polymorphism
        let _ = systemd_boot.install();
        let _ = grub.install();
    }
    
    #[test]
    fn test_bootloader_manager() {
        let bootloader = BootloaderFactory::create_bootloader("systemd-boot", "/boot/efi");
        let manager = BootloaderManager::new(bootloader);
        let result = manager.manage_bootloader("list");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_polymorphism() {
        // Both SystemdBoot and GRUB implement Bootloader trait
        let bootloaders: Vec<Box<dyn Bootloader>> = vec![
            Box::new(SystemdBoot::new("/boot/efi")),
            Box::new(Grub::new("/dev/sda")),
        ];
        
        // Can use them interchangeably
        for bootloader in bootloaders {
            let _ = bootloader.list_entries();
        }
    }
}
