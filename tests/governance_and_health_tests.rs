use std::path::Path;

#[test]
fn test_community_health_files_exist() {
    let required_files = [
        "CONTRIBUTING.md",
        "GOVERNANCE.md",
        "SUPPORT.md",
        "SECURITY.md",
        "ROADMAP.md",
        ".github/ISSUE_TEMPLATE/compiler_bug.yml",
        ".github/ISSUE_TEMPLATE/feature_request.yml",
    ];

    for file in required_files {
        assert!(
            Path::new(file).exists(),
            "Missing required community health file: {}",
            file
        );
    }
}
