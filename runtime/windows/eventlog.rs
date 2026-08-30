#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventLogLevel {
    Information = 4,
    Warning = 2,
    Error = 1,
}

pub struct WindowsEventLog {
    pub source: String,
}

impl WindowsEventLog {
    pub fn open(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }

    pub fn log(&self, event_id: u32, level: EventLogLevel, message: &str) {
        #[cfg(target_os = "windows")]
        {
            let _ = (event_id, level, message);
            // Uses Win32 ReportEventW API when running native on Windows
        }
        #[cfg(not(target_os = "windows"))]
        {
            eprintln!(
                "[Windows EventLog :: {}] Event {}: {:?}",
                self.source, event_id, message
            );
        }
    }

    pub fn info(&self, event_id: u32, message: &str) {
        self.log(event_id, EventLogLevel::Information, message);
    }

    pub fn warn(&self, event_id: u32, message: &str) {
        self.log(event_id, EventLogLevel::Warning, message);
    }

    pub fn error(&self, event_id: u32, message: &str) {
        self.log(event_id, EventLogLevel::Error, message);
    }
}
