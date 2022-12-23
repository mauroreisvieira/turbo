mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use crate::ffi::{nativeRunWithArgs, GoString};
use std::{ffi::CString, process};

// This function should not expanded. Please add any logic to
// `turborepo_lib::main` instead
fn main() {
    println!("Hello, world!");
}
