mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use crate::ffi::{nativeRunWithArgs, GoString};
use anyhow::Result;
use std::{env, ffi::CString};

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
    // let exit_code = unsafe { nativeRunWithArgs(serialized_args) };
    // println!("exit_code: {}", exit_code);
    // Ok(exit_code.try_into()?)
    println!("native_run");
    println!("serialized_args: {:#?}", serialized_args);
    Ok(0)
}

// This function should not expanded. Please add any logic to
// `turborepo_lib::main` instead
fn main() {
    println!("Running...");
    let args: Vec<String> = env::args().collect();

    let action = if args.len() > 1 { &args[1] } else { "no-arg" };
    match action {
        "run-go" => {
            println!("run-go");
            let _ = native_run();
        }
        "run-rust" => {
            println!("run-rust");
            let result = 1 + 10 / 5;
            println!("Here's the result of some rust code {}", result);
        }
        _ => {
            println!("received unknown action '{}'", action);
        }
    }
    println!("Done.");
}
