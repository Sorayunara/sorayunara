use sorayunara::windows::{ServiceConfig, WindowsEventLog, WindowsSupportTier, WindowsSystemInfo};

#[test]
fn test_windows_server_detection() {
    let sys = WindowsSystemInfo::detect();
    assert_eq!(sys.architecture, "x86_64");
    assert_eq!(sys.target_triple, "x86_64-pc-windows-msvc");
    assert_eq!(sys.support_tier, WindowsSupportTier::Tier1Production);
    assert!(sys.build_number >= 10000);
}

#[test]
fn test_windows_service_configuration() {
    let svc = ServiceConfig::new(
        "SorayunaraApi",
        "C:\\Program Files\\Sorayunara\\bin\\sorayunara.exe",
    );
    assert_eq!(svc.name, "SorayunaraApi");
    assert_eq!(svc.start_type, "automatic");
    assert_eq!(svc.account, "LocalService");
    assert!(svc.auto_restart_on_failure);

    let install_cmd = svc.generate_install_command();
    assert!(install_cmd.contains("New-Service"));
    assert!(install_cmd.contains("SorayunaraApi"));

    let start_cmd = svc.generate_start_command();
    assert_eq!(start_cmd, "Start-Service -Name \"SorayunaraApi\"");
}

#[test]
fn test_windows_eventlog_api() {
    let log = WindowsEventLog::open("SorayunaraTestService");
    assert_eq!(log.source, "SorayunaraTestService");
    log.info(1001, "Service started successfully");
    log.warn(2001, "High memory load detected");
    log.error(3001, "Failed to bind listener on port 8080");
}
