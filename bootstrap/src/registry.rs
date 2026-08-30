#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct PackageMeta {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub checksum: String,
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AuditReport {
    pub total_dependencies: usize,
    pub vulnerabilities_found: usize,
    pub warnings: Vec<String>,
    pub is_clean: bool,
}

pub struct RegistryClient {
    pub known_packages: HashMap<String, PackageMeta>,
}

impl RegistryClient {
    pub fn new() -> Self {
        let mut known = HashMap::new();

        known.insert(
            "http".to_string(),
            PackageMeta {
                name: "http".to_string(),
                version: "1.2.0".to_string(),
                description: "High-performance HTTP/1.1, HTTP/2, and HTTP/3 Server & Client"
                    .to_string(),
                author: "Sorayunara Core Team <core@sorayunara.org>".to_string(),
                license: "MIT OR Apache-2.0".to_string(),
                checksum: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    .to_string(),
                dependencies: HashMap::new(),
            },
        );

        known.insert(
            "postgres".to_string(),
            PackageMeta {
                name: "postgres".to_string(),
                version: "0.8.4".to_string(),
                description: "Pure Sorayunara PostgreSQL driver with connection pooling"
                    .to_string(),
                author: "Sorayunara Database WG".to_string(),
                license: "MIT".to_string(),
                checksum: "cca72f0a9154460f9e160a04918f8e0d63fe83ec3d4df489f6d4d1252ccf784e"
                    .to_string(),
                dependencies: HashMap::new(),
            },
        );

        known.insert(
            "json".to_string(),
            PackageMeta {
                name: "json".to_string(),
                version: "2.0.1".to_string(),
                description: "Zero-copy streaming JSON parser and serializer".to_string(),
                author: "Sorayunara Standard WG".to_string(),
                license: "MIT".to_string(),
                checksum: "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
                    .to_string(),
                dependencies: HashMap::new(),
            },
        );

        known.insert(
            "redis".to_string(),
            PackageMeta {
                name: "redis".to_string(),
                version: "1.1.0".to_string(),
                description: "In-memory cache & Pub/Sub Redis driver".to_string(),
                author: "Sorayunara Ecosystem".to_string(),
                license: "MIT".to_string(),
                checksum: "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
                    .to_string(),
                dependencies: HashMap::new(),
            },
        );

        known.insert(
            "jwt".to_string(),
            PackageMeta {
                name: "jwt".to_string(),
                version: "1.0.0".to_string(),
                description: "HMAC and RSA JSON Web Token generator & validator".to_string(),
                author: "Sorayunara Security".to_string(),
                license: "MIT".to_string(),
                checksum: "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                    .to_string(),
                dependencies: HashMap::new(),
            },
        );

        Self {
            known_packages: known,
        }
    }

    pub fn search(&self, query: &str) -> Vec<&PackageMeta> {
        let q = query.to_lowercase();
        self.known_packages
            .values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&q) || pkg.description.to_lowercase().contains(&q)
            })
            .collect()
    }

    pub fn install_package(
        &self,
        pkg_name: &str,
        project_dir: &Path,
    ) -> Result<PackageMeta, String> {
        let meta = self
            .known_packages
            .get(pkg_name)
            .ok_or_else(|| {
                format!(
                    "Package '{}' not found in Sorayunara Registry (packages.sorayunara.org)",
                    pkg_name
                )
            })?
            .clone();

        let pkg_cache_dir = project_dir
            .join(".sorayunara")
            .join("packages")
            .join(&meta.name);
        fs::create_dir_all(&pkg_cache_dir)
            .map_err(|e| format!("Failed to create package cache: {}", e))?;

        // Update sorayunara.toml / aoi.toml / nami.toml / aether.toml manifest
        let manifest_path = if project_dir.join("sorayunara.toml").exists() {
            project_dir.join("sorayunara.toml")
        } else if project_dir.join("aoi.toml").exists() {
            project_dir.join("aoi.toml")
        } else if project_dir.join("nami.toml").exists() {
            project_dir.join("nami.toml")
        } else if project_dir.join("aether.toml").exists() {
            project_dir.join("aether.toml")
        } else {
            project_dir.join("sorayunara.toml")
        };

        let mut manifest_content = if manifest_path.exists() {
            fs::read_to_string(&manifest_path).unwrap_or_default()
        } else {
            "[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\n".to_string()
        };

        if !manifest_content.contains(&format!("{} =", meta.name)) {
            if !manifest_content.contains("[dependencies]") {
                manifest_content.push_str("\n[dependencies]\n");
            }
            manifest_content.push_str(&format!("{} = \"{}\"\n", meta.name, meta.version));
            let _ = fs::write(&manifest_path, manifest_content);
        }

        // Update sorayunara.lock / aoi.lock / nami.lock / aether.lock file with SHA-256 integrity hash
        let lockfile_path = if project_dir.join("sorayunara.lock").exists() {
            project_dir.join("sorayunara.lock")
        } else if project_dir.join("aoi.lock").exists() {
            project_dir.join("aoi.lock")
        } else if project_dir.join("nami.lock").exists() {
            project_dir.join("nami.lock")
        } else if project_dir.join("aether.lock").exists() {
            project_dir.join("aether.lock")
        } else {
            project_dir.join("sorayunara.lock")
        };

        let lock_entry = format!(
            "[[package]]\nname = \"{}\"\nversion = \"{}\"\nchecksum = \"{}\"\n\n",
            meta.name, meta.version, meta.checksum
        );
        let mut lock_content = fs::read_to_string(&lockfile_path).unwrap_or_default();
        if !lock_content.contains(&format!("name = \"{}\"", meta.name)) {
            lock_content.push_str(&lock_entry);
            let _ = fs::write(&lockfile_path, lock_content);
        }

        Ok(meta)
    }

    pub fn audit_project(&self, project_dir: &Path) -> AuditReport {
        let lockfile_path = if project_dir.join("sorayunara.lock").exists() {
            project_dir.join("sorayunara.lock")
        } else if project_dir.join("aoi.lock").exists() {
            project_dir.join("aoi.lock")
        } else if project_dir.join("nami.lock").exists() {
            project_dir.join("nami.lock")
        } else if project_dir.join("aether.lock").exists() {
            project_dir.join("aether.lock")
        } else {
            return AuditReport {
                total_dependencies: 0,
                vulnerabilities_found: 0,
                warnings: vec![
                    "No sorayunara.lock file found. Run 'sorayunara add' to lock dependencies."
                        .to_string(),
                ],
                is_clean: true,
            };
        };

        let lock_content = fs::read_to_string(lockfile_path).unwrap_or_default();
        let total_deps = lock_content.matches("[[package]]").count();

        AuditReport {
            total_dependencies: total_deps,
            vulnerabilities_found: 0,
            warnings: Vec::new(),
            is_clean: true,
        }
    }

    pub fn dependency_tree(&self, project_dir: &Path) -> String {
        let manifest_path = if project_dir.join("sorayunara.toml").exists() {
            project_dir.join("sorayunara.toml")
        } else if project_dir.join("aoi.toml").exists() {
            project_dir.join("aoi.toml")
        } else if project_dir.join("nami.toml").exists() {
            project_dir.join("nami.toml")
        } else {
            project_dir.join("aether.toml")
        };

        let pkg_name = if let Ok(content) = fs::read_to_string(&manifest_path) {
            content
                .lines()
                .find(|l| l.trim_start().starts_with("name ="))
                .and_then(|l| l.split('=').nth(1))
                .map(|s| s.trim().trim_matches('"').to_string())
                .unwrap_or_else(|| "app".to_string())
        } else {
            "app".to_string()
        };

        let mut tree = format!("{} v0.1.0\n", pkg_name);
        let lockfile_path = if project_dir.join("sorayunara.lock").exists() {
            project_dir.join("sorayunara.lock")
        } else if project_dir.join("aoi.lock").exists() {
            project_dir.join("aoi.lock")
        } else if project_dir.join("nami.lock").exists() {
            project_dir.join("nami.lock")
        } else {
            project_dir.join("aether.lock")
        };

        if let Ok(lock_content) = fs::read_to_string(&lockfile_path) {
            let pkgs: Vec<&str> = lock_content
                .lines()
                .filter(|l| l.trim_start().starts_with("name ="))
                .map(|l| l.split('=').nth(1).unwrap().trim().trim_matches('"'))
                .collect();

            for (idx, p) in pkgs.iter().enumerate() {
                let is_last = idx == pkgs.len() - 1;
                let branch = if is_last { "└──" } else { "├──" };
                let ver = self
                    .known_packages
                    .get(*p)
                    .map(|m| m.version.as_str())
                    .unwrap_or("0.1.0");
                tree.push_str(&format!("  {} {} v{}\n", branch, p, ver));
            }
        }
        tree
    }
}
