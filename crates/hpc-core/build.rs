// crates/hpc-core/build.rs
use std::process::Command;

fn main() {
    let sha = cmd_str(&["git", "rev-parse", "HEAD"]);
    let br = cmd_str(&["git", "rev-parse", "--abbrev-ref", "HEAD"]);
    let dirty = cmd_str(&["git", "status", "--porcelain"]).map(|s| {
        if s.is_empty() {
            "0".to_string()
        } else {
            "1".to_string()
        }
    });

    if let Some(s) = sha {
        println!("cargo:rustc-env=GIT_SHA={}", s);
    }
    if let Some(s) = br {
        println!("cargo:rustc-env=GIT_BRANCH={}", s);
    }
    if let Some(s) = dirty {
        println!("cargo:rustc-env=GIT_DIRTY={}", s);
    }

    println!("cargo:rustc-check-cfg=cfg(hpc_core_dev)");
}

fn cmd_str(cmd: &[&str]) -> Option<String> {
    Command::new(cmd[0])
        .args(&cmd[1..])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
}
