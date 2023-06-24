use std::process::Command;

pub fn execute() {
    //println!("command execute here");
    let output = Command::new("ipconfig")
        .current_dir("") //TODO
        .output()
        .expect("failed to execute process");

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