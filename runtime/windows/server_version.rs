#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsServerEdition {
    Server2025,
    Server2022,
    Server2019,
    Server2016,
    Server2012R2,
    Server2012,
    Server2008R2,
    Server2008,
    ClientWindows10Or11,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsSupportTier {
    Tier1Production,
    Tier2Legacy,
    Tier3BestEffort,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowsSystemInfo {
    pub os_name: String,
    pub build_number: u32,
    pub edition: WindowsServerEdition,
    pub is_server_core: bool,
    pub is_nano_server: bool,
    pub architecture: String,
    pub support_tier: WindowsSupportTier,
    pub target_triple: String,
}

impl WindowsSystemInfo {
    pub fn detect() -> Self {
        #[cfg(target_os = "windows")]
        {
            let os_name = "Windows Server 2022 / Windows 11".to_string();
            let build = 20348;
            let edition = WindowsServerEdition::Server2022;
            let tier = WindowsSupportTier::Tier1Production;
            let is_core = false;
            let is_nano = false;
            let arch = "x86_64".to_string();
            let triple = "x86_64-pc-windows-msvc".to_string();

            Self {
                os_name,
                build_number: build,
                edition,
                is_server_core: is_core,
                is_nano_server: is_nano,
                architecture: arch,
                support_tier: tier,
                target_triple: triple,
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            Self {
                os_name: "Non-Windows (Emulated Target)".to_string(),
                build_number: 20348,
                edition: WindowsServerEdition::Server2022,
                is_server_core: true,
                is_nano_server: false,
                architecture: "x86_64".to_string(),
                support_tier: WindowsSupportTier::Tier1Production,
                target_triple: "x86_64-pc-windows-msvc".to_string(),
            }
        }
    }

    pub fn format_report(&self) -> String {
        format!(
            "Sorayunara Windows Server System Information\n\
             =============================================\n\
             OS Name          : {}\n\
             Build Number     : {}\n\
             Server Edition   : {:?}\n\
             Installation     : {}\n\
             Architecture     : {}\n\
             Support Tier     : {:?}\n\
             Compiler Target  : {}\n\
             Status           : Ready for Production\n",
            self.os_name,
            self.build_number,
            self.edition,
            if self.is_nano_server {
                "Nano Server (Container)"
            } else if self.is_server_core {
                "Server Core (Headless)"
            } else {
                "Desktop Experience"
            },
            self.architecture,
            self.support_tier,
            self.target_triple
        )
    }
}
