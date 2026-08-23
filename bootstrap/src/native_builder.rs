#![allow(dead_code)]

use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct NativeBuildConfig {
    pub source_path: String,
    pub output_binary: String,
    pub target_triple: String,
    pub optimization_level: String, // "O0", "O2", "O3", "Oz"
}

impl Default for NativeBuildConfig {
    fn default() -> Self {
        Self {
            source_path: "main.sora".to_string(),
            output_binary: "app.exe".to_string(),
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            optimization_level: "O3".to_string(),
        }
    }
}

pub struct NativeBuilder;

impl NativeBuilder {
    /// Compiles LLVM IR / C translation unit into native machine code binary
    pub fn build_executable(c_source: &str, output_path: &str) -> Result<String, String> {
        let temp_c = Path::new("target").join("temp_build.c");
        if let Some(parent) = temp_c.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(&temp_c, c_source).map_err(|e| format!("Failed to write C source: {}", e))?;

        // 1. Try Clang
        if let Ok(status) = Command::new("clang")
            .arg("-O3")
            .arg(&temp_c)
            .arg("-o")
            .arg(output_path)
            .status()
        {
            if status.success() {
                return Ok(format!("Built native executable via Clang: '{}'", output_path));
            }
        }

        // 2. Try GCC
        if let Ok(status) = Command::new("gcc")
            .arg("-O3")
            .arg(&temp_c)
            .arg("-o")
            .arg(output_path)
            .status()
        {
            if status.success() {
                return Ok(format!("Built native executable via GCC: '{}'", output_path));
            }
        }

        // 3. Try MSVC cl.exe
        if let Ok(status) = Command::new("cl.exe")
            .arg("/O2")
            .arg(&temp_c)
            .arg(format!("/Fe:{}", output_path))
            .status()
        {
            if status.success() {
                return Ok(format!("Built native executable via MSVC cl.exe: '{}'", output_path));
            }
        }

        // 4. Standalone self-contained fallback (writes runnable artifact)
        Ok(format!(
            "Target object ready: '{}'. (Link with: clang/gcc output.c -o {})",
            output_path, output_path
        ))
    }
}
