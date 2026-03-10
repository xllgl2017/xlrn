use std::{env, fs};
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    let typ = if cfg!(feature = "dylib") { "dylib" } else { "static" };
    if typ == "static" {
        println!("cargo:rustc-link-search={}/lib/{}/{}", manifest_dir, target_os, target_env);
    }
    println!("cargo:rustc-link-lib={}=xlnt", typ);
    match (target_os.as_str(), target_env.as_str()) {
        ("windows", "msvc") => {
            let out_dir = env::var("OUT_DIR").unwrap();
            let path = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();
            let dll_src = format!("{}/lib/{}/{}/xlnt.dll", manifest_dir, target_os, target_env);
            let dll_dst =path.join("xlnt.dll");
            fs::copy(&dll_src, &dll_dst).expect("Failed to copy DLL");

            //println!("cargo:rustc-link-lib=advapi32")
        }
        ("windows", "gnu") => {
            // println!("cargo:rustc-link-lib=dylib=advapi32");
            // println!("cargo:rustc-link-lib=stdc++");
        }
        ("linux", "gnu") => println!("cargo:rustc-link-lib=dylib=stdc++"),
        (_, _) => {}
    }
}
