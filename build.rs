use std::env;
use fs_extra::dir::{copy, CopyOptions};
use std::process::Command;

fn throw_error(message: &str) {
    panic!("\x1B[31m{message}\x1B[0m");
}

fn check_program(program: &str) {
    if Command::new(program).arg("--help").status().is_err() {
        throw_error(&format!("{program} is required but was not found in PATH"));
    }
}

fn run_git(cmd: &mut Command, err: &str) {
    let status = cmd.status().expect("failed to execute git");
    if !status.success() {
        throw_error(&format!("{err}: {status}"));
    }
}

fn main() {
    if cfg!(not(target_os = "macos")) {
        throw_error(
            r#"
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
❌ SwiftBridge requires macOS

This crate wraps Apple APIs and cannot be built
on non-macOS platforms.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"#
        );
    }

    check_program("git");
    check_program("xcodebuild");

    let currentdir = env::current_dir().expect("failed to get current dir");
    let swiftdir = currentdir.join("../").join("target/");
    let repo_name = "NativeUI-swift-bridge";
    let repo_dir = swiftdir.join(repo_name);

    if repo_dir.exists() {
        run_git(
            Command::new("git")
                .args(["pull", "--ff-only"])
                .current_dir(&repo_dir),
            "git pull failed",
        );
    } else {
        run_git(
            Command::new("git")
                .args([
                    "clone",
                    "https://github.com/aellul27/NativeUI-swift-bridge",
                ])
                .current_dir(&swiftdir),
            "git clone failed",
        );
    }

    // Build the framework
    let build_status = Command::new("xcodebuild")
        .current_dir(&repo_dir)
        .status()
        .expect("failed to execute xcodebuild");

    if !build_status.success() {
        throw_error("xcodebuild failed");
    }

    // Copy framework to target directory
    let source = repo_dir.join("build/Release/swiftbridge.framework");
    let destination_dir = swiftdir.clone();

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    copy(&source, &destination_dir, &options)
        .expect("failed to copy framework directory");

    println!("Copied swiftbridge.framework to target/");
}
