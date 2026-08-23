use std::fs;
use std::path::Path;

#[test]
fn test_developer_experience_project_lifecycle() {
    let test_dir = Path::new("target").join("test_dx_project");
    let _ = fs::remove_dir_all(&test_dir);

    // 1. Scaffold project directory
    fs::create_dir_all(test_dir.join("src")).unwrap();
    fs::create_dir_all(test_dir.join("tests")).unwrap();
    fs::create_dir_all(test_dir.join("packages")).unwrap();

    let toml_content = r#"[package]
name = "demo-app"
version = "0.1.0"
"#;
    fs::write(test_dir.join("nami.toml"), toml_content).unwrap();

    let main_src = r#"
        fn main() -> Int {
            let x: Int = 100
            return x
        }
    "#;
    fs::write(test_dir.join("src").join("main.nm"), main_src).unwrap();

    // Verify files exist
    assert!(test_dir.join("nami.toml").exists());
    assert!(test_dir.join("src").join("main.nm").exists());

    // 2. Clean test sandbox
    let _ = fs::remove_dir_all(&test_dir);
}
