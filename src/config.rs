use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn ready_vars() {
    //reads config.json and prepares environmental variables
    let mut file = match File::open("config.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open config.json: {}", error);
            return;
        }
    };
    // Read the contents of the file into a string
    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        println!("Failed to read config.json: {}", error);
        return;
    }
    // Print the entire contents of the file
    //println!("File contents:\n{}", contents);

    //Make JSON string into HashMap
    let j_config: HashMap<String, String> = serde_json::from_str(&contents).unwrap();
    //Put every key-value pair from hashmap into environment variables
    for (key, value) in &j_config {
        // Following line prints all key-value pairs just before setting them as environment variables
        //println!("{}: {}", key, value);
        env::set_var(key, value);
    }
    
}