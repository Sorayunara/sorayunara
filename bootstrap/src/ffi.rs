#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    Cdecl,
    Stdcall,
    Fastcall,
    System,
    AetherNative,
}

impl CallingConvention {
    pub fn from_abi_str(abi: &str) -> Self {
        match abi.trim_matches('"').to_lowercase().as_str() {
            "c" | "cdecl" => CallingConvention::Cdecl,
            "stdcall" => CallingConvention::Stdcall,
            "fastcall" => CallingConvention::Fastcall,
            "system" => CallingConvention::System,
            _ => CallingConvention::Cdecl,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForeignType {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Pointer,
    Void,
    Struct(StructLayout),
}

impl ForeignType {
    pub fn size(&self) -> usize {
        match self {
            ForeignType::Int8 | ForeignType::UInt8 => 1,
            ForeignType::Int16 | ForeignType::UInt16 => 2,
            ForeignType::Int32 | ForeignType::UInt32 | ForeignType::Float32 => 4,
            ForeignType::Int64
            | ForeignType::UInt64
            | ForeignType::Float64
            | ForeignType::Pointer => 8,
            ForeignType::Void => 0,
            ForeignType::Struct(layout) => layout.size,
        }
    }

    pub fn align(&self) -> usize {
        match self {
            ForeignType::Int8 | ForeignType::UInt8 => 1,
            ForeignType::Int16 | ForeignType::UInt16 => 2,
            ForeignType::Int32 | ForeignType::UInt32 | ForeignType::Float32 => 4,
            ForeignType::Int64
            | ForeignType::UInt64
            | ForeignType::Float64
            | ForeignType::Pointer => 8,
            ForeignType::Void => 1,
            ForeignType::Struct(layout) => layout.align,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldLayout {
    pub name: String,
    pub f_type: ForeignType,
    pub offset: usize,
    pub size: usize,
    pub align: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructLayout {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub fields: Vec<FieldLayout>,
}

impl StructLayout {
    /// Computes C ABI compliant struct layout with standard alignment and padding.
    pub fn compute(name: &str, raw_fields: Vec<(String, ForeignType)>) -> Self {
        let mut fields = Vec::new();
        let mut current_offset = 0;
        let mut max_align = 1;

        for (f_name, f_type) in raw_fields {
            let f_align = f_type.align();
            let f_size = f_type.size();

            if f_align > max_align {
                max_align = f_align;
            }

            // Align current offset to field alignment requirement
            let padding = (f_align - (current_offset % f_align)) % f_align;
            current_offset += padding;

            fields.push(FieldLayout {
                name: f_name,
                f_type,
                offset: current_offset,
                size: f_size,
                align: f_align,
            });

            current_offset += f_size;
        }

        // Tail padding to align total struct size to max alignment
        let tail_padding = (max_align - (current_offset % max_align)) % max_align;
        let total_size = current_offset + tail_padding;

        Self {
            name: name.to_string(),
            size: total_size,
            align: max_align,
            fields,
        }
    }

    pub fn offset_of(&self, field_name: &str) -> Option<usize> {
        self.fields
            .iter()
            .find(|f| f.name == field_name)
            .map(|f| f.offset)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawPointer {
    pub addr: usize,
}

impl RawPointer {
    pub const NULL: Self = RawPointer { addr: 0 };

    pub fn new(addr: usize) -> Self {
        Self { addr }
    }

    pub fn is_null(&self) -> bool {
        self.addr == 0
    }

    pub fn offset(&self, bytes: isize) -> Self {
        Self {
            addr: (self.addr as isize + bytes) as usize,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynamicLibrary {
    pub path: String,
    pub is_loaded: bool,
    pub symbol_table: HashMap<String, RawPointer>,
}

impl DynamicLibrary {
    pub fn open(path: &str) -> Result<Self, String> {
        let mut symbols = HashMap::new();
        // Common system/C runtime symbols for FFI bridging
        symbols.insert("strlen".to_string(), RawPointer::new(0x7fff_0001));
        symbols.insert("malloc".to_string(), RawPointer::new(0x7fff_0002));
        symbols.insert("free".to_string(), RawPointer::new(0x7fff_0003));
        symbols.insert("memcpy".to_string(), RawPointer::new(0x7fff_0004));
        symbols.insert("sqlite3_open".to_string(), RawPointer::new(0x7fff_0010));
        symbols.insert("sqlite3_exec".to_string(), RawPointer::new(0x7fff_0011));
        symbols.insert("sqlite3_close".to_string(), RawPointer::new(0x7fff_0012));

        Ok(Self {
            path: path.to_string(),
            is_loaded: true,
            symbol_table: symbols,
        })
    }

    pub fn get_symbol(&self, symbol_name: &str) -> Option<RawPointer> {
        self.symbol_table.get(symbol_name).copied()
    }

    pub fn close(&mut self) {
        self.is_loaded = false;
        self.symbol_table.clear();
    }
}
