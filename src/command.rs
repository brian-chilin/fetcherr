use std::process::Command;
use std::env;

pub fn execute() {
    let mut output = Command::new(
            "git"
        )
        .current_dir(env::var("proj").unwrap())
        .arg("pull")
        .output()
        .expect("Failed to execute git pull");
    // Check if the command was successful
    if output.status.success() {
        // Print the output
        let stdout = String::from_utf8_lossy(&output.stdout);
        //TODO figure out how to make this not hang. perhaps use spawn() instead of output()
        println!("Command output:\n{}", stdout);
    } else {
        // Print the error message
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed:\n{}", stderr);
    }

    // execute: cargo stable build --release --manifest-path="\path\to\rust_project\Cargo.toml"
    output = Command::new(
            "cargo"
        )
        .env("RUSTUP_HOME", env::var("rustup_home").unwrap())
        .env("RUSTUP_TOOLCHAIN", "STABLE")
        .arg("stable")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path=".to_owned() + &env::var("proj").unwrap() + "/Cargo.toml")
        .output()
        .expect("Failed to execute cargo build");

    // Check if the command was successful
    if output.status.success() {
        // Print the output
        let stdout = String::from_utf8_lossy(&output.stdout);
        //TODO figure out how to make this not hang. perhaps use spawn() instead of output()
        println!("Command output:\n{}", stdout);
    } else {
        // Print the error message
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed:\n{}", stderr);
    }

    // execute: systemctl restart systemctl_service
    output = Command::new(
        "systemctl"
        )
        .arg("restart")
        .arg(env::var("systemctl_service").unwrap())
        .output()
        .expect("Failed to execute");
    // Check if the command was successful
    if output.status.success() {
        // Print the output
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command output:\n{}", stdout);
    } else {
        // Print the error message
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed:\n{}", stderr);
    }

}