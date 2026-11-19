// SigmaOS Bootloader CLI
// Command-Query Separation: Read operations separate from write operations
// Interface Segregation: Small, focused interfaces

use sigmaos_bootloader::{BootloaderFactory, BootloaderManager};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("Usage: sigmaos-bootloader <bootloader-type> <action> [config]");
        println!("Bootloader types: systemd-boot, grub");
        println!("Actions: install, uninstall, list, add-entry, remove-entry, set-default");
        return;
    }
    
    let bootloader_type = &args[1];
    let action = &args[2];
    let config = if args.len() > 3 { &args[3] } else { "/boot/efi" };
    
    // Factory Pattern - create appropriate bootloader
    let bootloader = BootloaderFactory::create_bootloader(bootloader_type, config);
    
    // Manager Pattern - manage bootloader lifecycle
    let manager = BootloaderManager::new(bootloader);
    
    match manager.manage_bootloader(action) {
        Ok(_) => println!("Action '{}' completed successfully", action),
        Err(e) => eprintln!("Error: {}", e),
    }
}
