use sorayunara::registry::RegistryClient;
use std::fs;
use std::path::Path;

#[test]
fn test_package_architecture_scaffolding_and_tree() {
    let test_dir = Path::new("target").join("test_pkg_arch");
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir).unwrap();

    // 1. Scaffold full Sorayunara project architecture
    fs::create_dir_all(test_dir.join("src")).unwrap();
    fs::create_dir_all(test_dir.join("tests")).unwrap();
    fs::create_dir_all(test_dir.join("examples")).unwrap();
    fs::create_dir_all(test_dir.join("benches")).unwrap();

    let toml_content = r#"[package]
name = "my-server"
version = "1.0.0"

[dependencies]
http = "1.2.0"
postgres = "0.8.4"
json = "2.0.1"
"#;
    fs::write(test_dir.join("sorayunara.toml"), toml_content).unwrap();

    let lock_content = r#"version = 1

[[package]]
name = "http"
version = "1.2.0"
checksum = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

[[package]]
name = "postgres"
version = "0.8.4"
checksum = "cca72f0a9154460f9e160a04918f8e0d63fe83ec3d4df489f6d4d1252ccf784e"

[[package]]
name = "json"
version = "2.0.1"
checksum = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
"#;
    fs::write(test_dir.join("sorayunara.lock"), lock_content).unwrap();

    fs::write(test_dir.join("build.sora"), "// build.sora\n").unwrap();
    fs::write(test_dir.join("src").join("main.sora"), "fn main() {}\n").unwrap();
    fs::write(test_dir.join("examples").join("basic.sora"), "fn main() {}\n").unwrap();
    fs::write(test_dir.join("benches").join("bench_main.sora"), "fn main() {}\n").unwrap();

    // Verify all structure elements exist
    assert!(test_dir.join("sorayunara.toml").exists());
    assert!(test_dir.join("sorayunara.lock").exists());
    assert!(test_dir.join("build.sora").exists());
    assert!(test_dir.join("src").join("main.sora").exists());
    assert!(test_dir.join("examples").join("basic.sora").exists());
    assert!(test_dir.join("benches").join("bench_main.sora").exists());

    // 2. Test Dependency Tree visualization
    let client = RegistryClient::new();
    let tree = client.dependency_tree(&test_dir);
    assert!(tree.contains("my-server v0.1.0"));
    assert!(tree.contains("http v1.2.0"));
    assert!(tree.contains("postgres v0.8.4"));
    assert!(tree.contains("json v2.0.1"));

    let _ = fs::remove_dir_all(&test_dir);
}
