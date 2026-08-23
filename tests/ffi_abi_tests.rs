use sorayunara::ffi::{CallingConvention, DynamicLibrary, ForeignType, RawPointer, StructLayout};
use sorayunara::lexer::tokenize;
use sorayunara::llvm_backend::emit_llvm_ir;
use sorayunara::parser::parse;
use sorayunara::semantics::check_semantics;

#[test]
fn test_calling_convention_and_abi() {
    assert_eq!(CallingConvention::from_abi_str("C"), CallingConvention::Cdecl);
    assert_eq!(CallingConvention::from_abi_str("stdcall"), CallingConvention::Stdcall);
    assert_eq!(CallingConvention::from_abi_str("fastcall"), CallingConvention::Fastcall);
    assert_eq!(CallingConvention::from_abi_str("system"), CallingConvention::System);
}

#[test]
fn test_c_struct_layout_and_alignment_padding() {
    // struct Example {
    //     a: u8,   // offset 0, size 1, padding 3
    //     b: u32,  // offset 4, size 4
    //     c: u16,  // offset 8, size 2, tail padding 2
    // } -> total size 12, max align 4
    let raw_fields = vec![
        ("a".to_string(), ForeignType::UInt8),
        ("b".to_string(), ForeignType::UInt32),
        ("c".to_string(), ForeignType::UInt16),
    ];

    let layout = StructLayout::compute("Example", raw_fields);
    assert_eq!(layout.align, 4);
    assert_eq!(layout.size, 12);
    assert_eq!(layout.offset_of("a"), Some(0));
    assert_eq!(layout.offset_of("b"), Some(4));
    assert_eq!(layout.offset_of("c"), Some(8));
}

#[test]
fn test_raw_pointer_and_dynamic_library_loading() {
    let p = RawPointer::new(1024);
    assert!(!p.is_null());
    let offset_p = p.offset(16);
    assert_eq!(offset_p.addr, 1040);
    assert!(RawPointer::NULL.is_null());

    let mut lib = DynamicLibrary::open("sqlite3.dll").unwrap();
    assert!(lib.is_loaded);
    let sym = lib.get_symbol("sqlite3_open");
    assert!(sym.is_some());
    assert!(!sym.unwrap().is_null());

    lib.close();
    assert!(!lib.is_loaded);
}

#[test]
fn test_extern_c_block_in_compiler_pipeline() {
    let source = r#"
        extern "C" {
            fn strlen(ptr: String) -> Int
            fn sqlite3_open(path: String) -> Int
        }

        fn main() -> Int {
            return 0
        }
    "#;
    let tokens = tokenize(source).unwrap();
    let program = parse(tokens).unwrap();
    assert!(check_semantics(&program).is_ok());

    let llvm_ir = emit_llvm_ir(&program);
    assert!(llvm_ir.contains("declare i64 @strlen(i8*)"));
    assert!(llvm_ir.contains("declare i64 @sqlite3_open(i8*)"));
}
