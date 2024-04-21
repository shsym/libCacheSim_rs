use std::process::Command;
use std::env;
use std::path::PathBuf;

// fn main() {
//     let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
//     let glib_include_path = "/usr/include/glib-2.0";
//     let glib_config_include_path = "/usr/lib/x86_64-linux-gnu/glib-2.0/include";

//     let compile_status = Command::new("g++")
//     .args(&["-I", glib_include_path, "-I", glib_config_include_path, "src/TraceWrapper.cpp", "-c", "-o", &format!("{}/TraceWrapper.o", out_dir)])
//     .status();

//     match compile_status {
//         Ok(status) => {
//             if !status.success() {
//                 println!("Compilation failed. Error code: {:?}", status.code());
//                 panic!("See error details above for more information.");
//             }
//         },
//         Err(e) => {
//             panic!("Failed to execute g++: {}", e);
//         }
//     }


//     println!("Creating libTraceWrapper.a");
//     let ar_status = Command::new("ar")
//         .args(&["rcs", "libTraceWrapper.a", "TraceWrapper.o"])
//         .current_dir(&PathBuf::from(out_dir.clone()))
//         .status()
//         .expect("Failed to create static library");
//     println!("ar status: {:?}", ar_status);

//     println!("Setting link-search path to {}", out_dir);
//     println!("cargo:rustc-link-search=native={}", out_dir);
//     println!("cargo:rustc-link-lib=static=TraceWrapper");
//     println!("cargo:rustc-link-search=native=/usr/local/lib");
//     println!("cargo:rustc-link-lib=static=libCacheSim");
//     println!("cargo:rustc-link-lib=dylib=glib-2.0");
//     println!("cargo:rustc-link-lib=dylib=zstd");
//     println!("cargo:rustc-link-lib=stdc++");
// }

fn main() {
    let lib_dir = "../cacheReaderLibRs/target/release";
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=static=cache_reader");
}
