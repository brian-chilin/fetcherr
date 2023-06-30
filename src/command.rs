use std::process::Command;
use std::env;

pub fn execute() {
    //println!("command execute here");

    /*
    let output = Command::new("testrrr.exe")
        .current_dir(
            env::var("directory").unwrap() + "\\target\\debug"
        )
        .output()
        .expect("failed to execute process");
    */
    println!("{}", env::var("proj_cargo_toml").unwrap().replace("\\", "\\\\") );
    let output = Command::new(
        //env::var("cargo_directory").unwrap() + "\\cargo"
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
        println!("Command output:\n{}", stdout);
    } else {
        // Print the error message
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed:\n{}", stderr);
    }

}