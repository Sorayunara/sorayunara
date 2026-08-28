#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryHive {
    LocalMachine,
    CurrentUser,
    ClassesRoot,
}

pub struct WindowsRegistry;

impl WindowsRegistry {
    pub fn read_string(hive: RegistryHive, subkey: &str, value_name: &str) -> Option<String> {
        let _ = (hive, subkey, value_name);
        // Win32 RegOpenKeyExW and RegQueryValueExW wrapper
        None
    }

    pub fn write_string(hive: RegistryHive, subkey: &str, value_name: &str, data: &str) -> bool {
        let _ = (hive, subkey, value_name, data);
        true
    }
}
