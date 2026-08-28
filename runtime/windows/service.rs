#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    StartPending,
    StopPending,
    Running,
    ContinuePending,
    PausePending,
    Paused,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceConfig {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub executable_path: String,
    pub start_type: String, // automatic, manual, disabled
    pub account: String,    // LocalService, LocalSystem, custom
    pub auto_restart_on_failure: bool,
    pub restart_delay_sec: u32,
}

impl ServiceConfig {
    pub fn new(name: &str, executable_path: &str) -> Self {
        Self {
            name: name.to_string(),
            display_name: format!("{} Service", name),
            description: format!("Sorayunara Server Runtime Managed Service ({})", name),
            executable_path: executable_path.to_string(),
            start_type: "automatic".to_string(),
            account: "LocalService".to_string(),
            auto_restart_on_failure: true,
            restart_delay_sec: 30,
        }
    }

    pub fn generate_install_command(&self) -> String {
        format!(
            "New-Service -Name \"{}\" -BinaryPathName \"{}\" -DisplayName \"{}\" -Description \"{}\" -StartupType {}",
            self.name, self.executable_path, self.display_name, self.description, self.start_type
        )
    }

    pub fn generate_start_command(&self) -> String {
        format!("Start-Service -Name \"{}\"", self.name)
    }

    pub fn generate_stop_command(&self) -> String {
        format!("Stop-Service -Name \"{}\"", self.name)
    }

    pub fn generate_uninstall_command(&self) -> String {
        format!("sc.exe delete \"{}\"", self.name)
    }
}
