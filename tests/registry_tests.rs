use sorayunara::registry::RegistryClient;
use std::fs;
use std::path::Path;

#[test]
fn test_registry_search_packages() {
    let client = RegistryClient::new();
    let http_results = client.search("http");
    assert!(!http_results.is_empty());
    assert_eq!(http_results[0].name, "http");

    let postgres_results = client.search("postgres");
    assert!(!postgres_results.is_empty());
    assert_eq!(postgres_results[0].name, "postgres");
}

#[test]
fn test_registry_install_and_lockfile_integrity() {
    let test_dir = Path::new("target").join("test_registry_sandbox");
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir).unwrap();

    let client = RegistryClient::new();
    let meta = client.install_package("json", &test_dir).unwrap();
    assert_eq!(meta.name, "json");
    assert_eq!(meta.version, "2.0.1");

    // Verify sorayunara.toml
    let toml_content = fs::read_to_string(test_dir.join("sorayunara.toml")).unwrap();
    assert!(toml_content.contains("json = \"2.0.1\""));

    // Verify sorayunara.lock
    let lock_content = fs::read_to_string(test_dir.join("sorayunara.lock")).unwrap();
    assert!(lock_content.contains("name = \"json\""));
    assert!(lock_content.contains(&meta.checksum));

    // Audit project
    let audit_report = client.audit_project(&test_dir);
    assert_eq!(audit_report.total_dependencies, 1);
    assert_eq!(audit_report.vulnerabilities_found, 0);
    assert!(audit_report.is_clean);

    let _ = fs::remove_dir_all(&test_dir);
}
