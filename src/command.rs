use std::process::Command;
use std::env;

pub fn execute() {
    // execute: cargo build --manifest-path="\path\to\rust_project\Cargo.toml"
    let output = Command::new(
            "cargo"
        )
        .arg("build")
        .arg("--manifest-path=".to_owned() + &env::var("proj_cargo_toml").unwrap())
        .output()
        .expect("Failed to execute process");

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

}