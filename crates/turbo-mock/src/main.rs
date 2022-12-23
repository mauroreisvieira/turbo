mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use crate::ffi::{nativeRunWithArgs, GoString};
use anyhow::Result;
use std::{ffi::CString, process};

fn native_run() -> Result<i32> {
    let json = r#"
        {
            "version":false,
            "api":null,
            "color":false,
            "cpu_profile":null,
            "cwd":"/tmp/",
            "heap":null,
            "login":null,
            "no_color":false,
            "preflight":false,
            "team":null,
            "token":null,
            "trace":null,
            "verbosity":0,
            "check_for_update":false,
            "test_run":false,
            "run_args":null,
            "command":{
               "Run":{
                  "cache_dir":null,
                  "cache_workers":10,
                  "concurrency":null,
                  "continue_execution":false,
                  "dry_run":null,
                  "single_package":false,
                  "filter":[

                  ],
                  "force":false,
                  "global_deps":[

                  ],
                  "graph":null,
                  "ignore":[

                  ],
                  "include_dependencies":false,
                  "no_cache":false,
                  "no_daemon":false,
                  "no_deps":false,
                  "output_logs":"full",
                  "only":false,
                  "parallel":false,
                  "profile":null,
                  "remote_only":false,
                  "scope":[

                  ],
                  "since":null,
                  "tasks":[
                     "build"
                  ],
                  "pass_through_args":[

                  ]
               }
            }
         }"#;

    let cstring = CString::new(json)?;
    let n = cstring.as_bytes().len() as isize;

    let serialized_args = GoString {
        p: cstring.into_raw(),
        n,
    };
    let exit_code = unsafe { nativeRunWithArgs(serialized_args) };
    Ok(exit_code.try_into()?)
}

// This function should not expanded. Please add any logic to
// `turborepo_lib::main` instead
fn main() {
    println!("Hello, world 3!");
    native_run();
    println!("done!");
}
