use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn ready_vars() {
    //reads secrets.json and prepares environmental variables
    let mut file = match File::open("secrets.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open secrets.json: {}", error);
            return;
        }
    };
    // Read the contents of the file into a string
    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        println!("Failed to read secrets.json: {}", error);
        return;
    }

    // Print the contents of the file
    //println!("File contents:\n{}", contents);
    let j_secrets: HashMap<String, String> = serde_json::from_str(&contents).unwrap();
    //println!("j_secrets: {:?}", j_secrets);
    for (key, value) in &j_secrets {
        println!("{}: {}", key, value);
        env::set_var(key, value);
    }
    //env::set_var("url", OsString::from(j_secrets.get("url").unwrap()));

}