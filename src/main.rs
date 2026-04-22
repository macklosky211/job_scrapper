/// To use this program just create a websites.csv file.
/// Each line of the file should be a full url pointing to a website.
use colored::Colorize;
use scraper::{Html, Selector};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions, read_to_string},
    io::Write,
};

const FILE_PATH: &str = "websites.csv";
const HASH_FILE_PATH: &str = "hashes.csv";

fn main() {
    let urls: String = match read_to_string(FILE_PATH) {
        Ok(string) => string,
        Err(e) => {
            eprintln!("Failed to find the file '{}': {}", FILE_PATH, e);
            return;
        }
    };

    let hashes: String = match read_to_string(HASH_FILE_PATH) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read hashing file, '{}': {}", HASH_FILE_PATH, e);
            String::new()
        }
    };

    let mut hash_map: HashMap<String, String> = HashMap::new();
    for line in hashes.split('\n').filter(|l| !l.is_empty()) {
        if let Some((url, hash)) = line.split_once(',') {
            hash_map.insert(url.trim().to_string(), hash.trim().to_string());
        };
    }

    let urls: Vec<String> = urls
        .split('\n')
        .map(|s| format!("{}", s.trim()))
        .filter(|s| !s.is_empty())
        .collect();

    for url in &urls {
        let hash = hash_website(url);
        if hash_map.contains_key(url) {
            // this website has been seen before, so compare it

            if let Some(saved_hash) = hash_map.get(url) {
                if saved_hash == &hash {
                    println!("{} '{}'", "No change found for".red(), url);
                    continue;
                } else {
                    // hashes didnt match -- this is what im looking for
                    let out = format!("CHANGE FOUND FOR: '{}'", url).green();
                    println!("{out}");
                    hash_map.insert(url.to_string(), hash); // overwrite old hash
                }
            } else {
                // it shouldnt be possible to get here.
                panic!(
                    "A Url had some kind of invalid hash assosiated, somethings gone horribly wrong... {}",
                    url
                );
            }
        } else {
            // hash didnt exist at all.
            let out = format!("Creating new entry for '{}'", url).yellow();
            println!("{out}");
            hash_map.insert(url.to_string(), hash);
        }
    }

    let mut web_hashes: File = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(HASH_FILE_PATH)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to find hashes file '{}': {}", HASH_FILE_PATH, e);
            return;
        }
    };

    for (url, hash) in hash_map {
        if let Err(e) = writeln!(web_hashes, "{}, {}", url, hash) {
            eprintln!("Failed to save data for '{}': {}", url, e);
        }
    }
}

fn hash_str(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    hex::encode(hasher.finalize())
}

fn hash_website(url: &str) -> String {
    let raw_html = ureq::get(url)
        .call()
        .unwrap()
        .into_body()
        .read_to_string()
        .unwrap();

    let web_data = strip_html(raw_html.as_str());

    return hash_str(web_data.as_str());
}

fn strip_html(html: &str) -> String {
    let document = Html::parse_document(html);

    let selector = Selector::parse("body *:not(script):not(style):not(meta)").unwrap();
    let text = document
        .select(&selector)
        .flat_map(|el| el.text())
        .collect::<String>();

    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}
