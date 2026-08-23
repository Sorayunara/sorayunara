use sorayunara::lockfile::{SorayunaraLock, LockedPackage};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[test]
fn test_reproducible_lockfile_roundtrip() {
    let mut lock = SorayunaraLock::new();
    lock.packages.push(LockedPackage {
        name: "http".to_string(),
        version: "1.2.0".to_string(),
        source: "https://packages.sorayunara.org/packages/http/1.2.0".to_string(),
        checksum: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
        dependencies: vec!["json".to_string()],
    });
    lock.packages.push(LockedPackage {
        name: "json".to_string(),
        version: "2.0.1".to_string(),
        source: "https://packages.sorayunara.org/packages/json/2.0.1".to_string(),
        checksum: "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08".to_string(),
        dependencies: vec![],
    });

    let toml_str = lock.to_toml();
    let parsed_lock = SorayunaraLock::parse(&toml_str).unwrap();

    assert_eq!(parsed_lock.packages.len(), 2);
    assert_eq!(parsed_lock.packages[0].name, "http");
    assert_eq!(parsed_lock.packages[0].version, "1.2.0");
    assert_eq!(parsed_lock.packages[0].dependencies, vec!["json".to_string()]);
    assert_eq!(parsed_lock.packages[1].name, "json");
}

#[test]
fn test_reproducible_checksum_verification_and_restore() {
    let test_dir = Path::new("target").join("test_reproducible_sandbox");
    let _ = fs::remove_dir_all(&test_dir);
    fs::create_dir_all(&test_dir).unwrap();

    let mut lock = SorayunaraLock::new();
    lock.packages.push(LockedPackage {
        name: "postgres".to_string(),
        version: "0.8.4".to_string(),
        source: "https://packages.sorayunara.org/packages/postgres/0.8.4".to_string(),
        checksum: "cca72f0a9154460f9e160a04918f8e0d63fe83ec3d4df489f6d4d1252ccf784e".to_string(),
        dependencies: vec![],
    });

    let mut registry_sums = HashMap::new();
    registry_sums.insert(
        "postgres".to_string(),
        "cca72f0a9154460f9e160a04918f8e0d63fe83ec3d4df489f6d4d1252ccf784e".to_string(),
    );

    // Verify valid checksum
    assert!(lock.verify_checksums(&registry_sums).is_ok());

    // Tampered checksum should fail
    registry_sums.insert("postgres".to_string(), "tampered_hash_000000".to_string());
    assert!(lock.verify_checksums(&registry_sums).is_err());

    // Restore test
    let restored_count = lock.restore_reproducible_build(&test_dir).unwrap();
    assert_eq!(restored_count, 1);
    let pkg_info = test_dir
        .join(".sorayunara")
        .join("packages")
        .join("postgres")
        .join("0.8.4")
        .join("pkg.info");
    assert!(pkg_info.exists());
}
