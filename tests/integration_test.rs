use sigmaos_bootloader::*;

#[test]
fn component_manages_boot_entries() {
    let mut cfg = BootloaderConfig::new();
    cfg.add_entry(BootEntry { id: "0001".into(), title: "SigmaOS".into(),
        loader: "/EFI/sigmaos.efi".into(), options: vec![] }).unwrap();
    assert_eq!(cfg.entries.len(), 1);
}
