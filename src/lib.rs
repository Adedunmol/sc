use std::{self, fs::{self, File, OpenOptions}, path::{self, PathBuf}, io::{Read, Write}};
use serde_json::{Value::{self, Null}, json};
use clap::Parser;

#[derive(Parser)]
pub struct Config {
    /// Add a new word
    #[arg(short, long)]
    word: Option<String>,

    /// Path to the file to read from
    path: Option<String>,
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "../dictionary.json";
    let mut dictionary = load_dictionary()?;

    if let Some(word) = &config.word {
        add_key_to_json_file(file_path, word, dictionary)?
    } else if let Some(path) = &config.path {
        read_file_and_validate(path, &dictionary)? 
    }
    

    Ok(())
}

fn load_dictionary() -> Result<Value, Box<dyn std::error::Error>> {
    
    let content = fs::read_to_string("../dictionary.json").expect("Could not read the file");
    let dict = serde_json::from_str::<Value>(&content).expect("Could not parse the JSON data");
    

    Ok(dict)
}

fn word_in_dictionary(word: &str, dictionary: &Value) -> bool {

    let word_exists = dictionary[word.to_lowercase()] != Null;

    word_exists
}

fn read_file_and_validate(path: &str, dictionary: &Value) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut line_no = 1;
    let mut file = File::open(path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    for line in content.lines() {
        for word in line.split(&['-', ' ', ':', '@', ',', '.', ';', '?', '\''][..]) {
            if word.is_empty() {
                continue;
            }

            if !word_in_dictionary(word, dictionary) {
                println!("- Line {}: {} appears to be a typo", line_no, word);
            }
        }

        line_no += 1;
    }
    
    Ok(())
}

fn add_key_to_json_file(file_path: &str, key: &str, mut data: Value) -> Result<(), Box<dyn std::error::Error>> {

    if let Value::Object(map) = &mut data {
        map.insert(key.to_string(), json!(1));
    }

    // Serialize the modified data back into JSON format
    let updated_json = serde_json::to_string_pretty(&data)?;

    // Write the updated JSON data back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    file.write_all(updated_json.as_bytes())?;

    println!("New word added!");

    Ok(())
}