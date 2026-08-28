#![allow(dead_code)]

pub struct WindowsSecurity;

impl WindowsSecurity {
    pub fn is_running_as_admin() -> bool {
        #[cfg(target_os = "windows")]
        {
            // Win32 CheckTokenMembership check
            false
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    pub fn encrypt_dpapi(data: &[u8]) -> Result<Vec<u8>, String> {
        // Win32 CryptProtectData (DPAPI)
        Ok(data.to_vec())
    }

    pub fn decrypt_dpapi(encrypted: &[u8]) -> Result<Vec<u8>, String> {
        // Win32 CryptUnprotectData (DPAPI)
        Ok(encrypted.to_vec())
    }
}
